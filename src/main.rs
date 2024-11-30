use std::{env, thread, time::Duration};

use firefly::{Firefly, FireflyServiceSyncClient, Position, TFireflyServiceSyncClient};
use rand::random;
use thrift::{
    protocol::{TBinaryInputProtocol, TBinaryOutputProtocol},
    transport::{ReadHalf, TIoChannel, TTcpChannel, WriteHalf},
    OrderedFloat,
};

mod firefly;

struct GridSize {
    x: i16,
    y: i16,
}

struct EnvironmentInformation {
    firefly: Firefly,
    gridsize: GridSize,
    coupling_strength: f64,
    min_frequency: f64,
    max_frequency_deviation: f64,
}

fn main() {
    let EnvironmentInformation {
        mut firefly,
        gridsize,
        coupling_strength,
        min_frequency,
        max_frequency_deviation,
    } = get_environment_information().unwrap();

    let mut channel = TTcpChannel::new();
    channel.open("127.0.0.1:9090").unwrap(); // Adjust the host/port as needed
    println!("Started client on 127.0.0.1:9090 with data: {:?}", firefly);

    // Setup buffered transport for better performance
    let (i_transport, o_transport) = channel.split().unwrap();
    let (i_proto, o_proto) = (
        TBinaryInputProtocol::new(i_transport, true),
        TBinaryOutputProtocol::new(o_transport, true),
    );

    // Create a Thrift client instance
    let mut client = FireflyServiceSyncClient::new(i_proto, o_proto);

    let frequency = min_frequency + random::<f64>() * max_frequency_deviation;

    loop {
        update(
            &mut firefly,
            &gridsize,
            &frequency,
            &coupling_strength,
            &mut client,
        )
        .unwrap();
        //println!("Sending new phase: {:?}", firefly);
        client.send_phase_update(firefly.clone()).unwrap();
        thread::sleep(Duration::from_millis(10));
    }
}

fn update(
    state: &mut Firefly,
    gridsize: &GridSize,
    frequency: &f64,
    coupling_strength: &f64,
    client: &mut FireflyServiceSyncClient<
        TBinaryInputProtocol<ReadHalf<TTcpChannel>>,
        TBinaryOutputProtocol<WriteHalf<TTcpChannel>>,
    >,
) -> Result<(), Box<dyn std::error::Error>> {
    let above = Position {
        x: state.position.x,
        y: (state.position.y + 1) % gridsize.y,
    };
    let below = Position {
        x: state.position.x,
        y: match state.position.y - 1 {
            n if n < 0 => gridsize.y - 1,
            n => n,
        },
    };
    let right = Position {
        x: (state.position.x + 1) % gridsize.x,
        y: state.position.y,
    };
    let left = Position {
        x: match state.position.x - 1 {
            n if n < 0 => gridsize.x - 1,
            n => n,
        },
        y: state.position.y,
    };
    let neighbors = vec![above, below, right, left];

    // TODO: kind of inefficient because the rpc calls are blocking four times in a row
    let neighbor_phase_difference: f64 = neighbors
        .iter()
        .map(|pos| {
            let other_phase = client.get_phase_by_firefly_position((*pos).clone());
            match other_phase {
                Ok(phase) => (phase - state.phase).sin(),
                Err(err) => {
                    // println!(
                    //     "Can not get phase of other firefly! Maybe it is not yet instantiated? Error: {:?}", err
                    // );
                    0.
                }
            }
        })
        .sum();
    let phase_tick = (coupling_strength / 4.) * neighbor_phase_difference + frequency;
    let total_phase = 2.0 * std::f64::consts::PI;
    {
        state.phase += phase_tick;
        if state.phase >= OrderedFloat(total_phase) {
            state.phase -= total_phase;
        }
    }
    Ok(())
}

fn get_environment_information() -> Result<EnvironmentInformation, Box<dyn std::error::Error>> {
    let position_x;
    let position_y;
    let gridsize_x;
    let gridsize_y;
    let coupling_strength;
    let min_frequency;
    let max_frequency_deviation;
    match env::var("POSITION_X") {
        Ok(value) => {
            position_x = value.parse::<i16>()?;
        }
        Err(e) => panic!("Couldn't read POSITION_X: {}", e),
    }
    match env::var("POSITION_Y") {
        Ok(value) => {
            position_y = value.parse::<i16>()?;
        }
        Err(e) => panic!("Couldn't read POSITION_Y: {}", e),
    }
    match env::var("GRIDSIZE_X") {
        Ok(value) => {
            gridsize_x = value.parse::<i16>()?;
        }
        Err(e) => panic!("Couldn't read GRIDSIZE_X: {}", e),
    }
    match env::var("GRIDSIZE_Y") {
        Ok(value) => {
            gridsize_y = value.parse::<i16>()?;
        }
        Err(e) => panic!("Couldn't read GRIDSIZE_Y: {}", e),
    }
    match env::var("COUPLING_STRENGTH") {
        Ok(value) => {
            coupling_strength = value.parse::<f64>()?;
        }
        Err(e) => panic!("Couldn't read COUPLING_STRENGTH: {}", e),
    }
    match env::var("MIN_FREQUENCY") {
        Ok(value) => {
            min_frequency = value.parse::<f64>()?;
        }
        Err(e) => panic!("Couldn't read MIN_FREQUENCY: {}", e),
    }
    match env::var("MAX_FREQUENCY_DEVIATION") {
        Ok(value) => {
            max_frequency_deviation = value.parse::<f64>()?;
        }
        Err(e) => panic!("Couldn't read FREQUENCY_DEVIATION: {}", e),
    }
    let phase = OrderedFloat(rand::random::<f64>() * 2. * std::f64::consts::PI);
    Ok(EnvironmentInformation {
        firefly: Firefly {
            position: Position {
                x: position_x,
                y: position_y,
            },
            phase,
        },
        gridsize: GridSize {
            x: gridsize_x,
            y: gridsize_y,
        },
        coupling_strength,
        min_frequency,
        max_frequency_deviation,
    })
}
