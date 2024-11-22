// Autogenerated by Thrift Compiler (0.17.0)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![allow(clippy::too_many_arguments, clippy::type_complexity, clippy::vec_box)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{From, TryFrom};
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use thrift::OrderedFloat;
use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSerializable, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::server::TProcessor;

//
// Position
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
  pub x: i16,
  pub y: i16,
}

impl Position {
  pub fn new(x: i16, y: i16) -> Position {
    Position {
      x,
      y,
    }
  }
}

impl TSerializable for Position {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<Position> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<i16> = None;
    let mut f_2: Option<i16> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i16()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_i16()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("Position.x", &f_1)?;
    verify_required_field_exists("Position.y", &f_2)?;
    let ret = Position {
      x: f_1.expect("auto-generated code should have checked for presence of required fields"),
      y: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Position");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("x", TType::I16, 1))?;
    o_prot.write_i16(self.x)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("y", TType::I16, 2))?;
    o_prot.write_i16(self.y)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// Firefly
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Firefly {
  pub position: Position,
  pub phase: OrderedFloat<f64>,
}

impl Firefly {
  pub fn new(position: Position, phase: OrderedFloat<f64>) -> Firefly {
    Firefly {
      position,
      phase,
    }
  }
}

impl TSerializable for Firefly {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<Firefly> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Position> = None;
    let mut f_2: Option<OrderedFloat<f64>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = Position::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        2 => {
          let val = OrderedFloat::from(i_prot.read_double()?);
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("Firefly.position", &f_1)?;
    verify_required_field_exists("Firefly.phase", &f_2)?;
    let ret = Firefly {
      position: f_1.expect("auto-generated code should have checked for presence of required fields"),
      phase: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Firefly");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("position", TType::Struct, 1))?;
    self.position.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("phase", TType::Double, 2))?;
    o_prot.write_double(self.phase.into())?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// FireflyService service client
//

pub trait TFireflyServiceSyncClient {
  fn get_phase_by_firefly_position(&mut self, position: Position) -> thrift::Result<OrderedFloat<f64>>;
  fn send_phase_update(&mut self, firefly: Firefly) -> thrift::Result<()>;
}

pub trait TFireflyServiceSyncClientMarker {}

pub struct FireflyServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  _i_prot: IP,
  _o_prot: OP,
  _sequence_number: i32,
}

impl <IP, OP> FireflyServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  pub fn new(input_protocol: IP, output_protocol: OP) -> FireflyServiceSyncClient<IP, OP> {
    FireflyServiceSyncClient { _i_prot: input_protocol, _o_prot: output_protocol, _sequence_number: 0 }
  }
}

impl <IP, OP> TThriftClient for FireflyServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  fn i_prot_mut(&mut self) -> &mut dyn TInputProtocol { &mut self._i_prot }
  fn o_prot_mut(&mut self) -> &mut dyn TOutputProtocol { &mut self._o_prot }
  fn sequence_number(&self) -> i32 { self._sequence_number }
  fn increment_sequence_number(&mut self) -> i32 { self._sequence_number += 1; self._sequence_number }
}

impl <IP, OP> TFireflyServiceSyncClientMarker for FireflyServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}

impl <C: TThriftClient + TFireflyServiceSyncClientMarker> TFireflyServiceSyncClient for C {
  fn get_phase_by_firefly_position(&mut self, position: Position) -> thrift::Result<OrderedFloat<f64>> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("getPhaseByFireflyPosition", TMessageType::Call, self.sequence_number());
        let call_args = FireflyServiceGetPhaseByFireflyPositionArgs { position };
        self.o_prot_mut().write_message_begin(&message_ident)?;
        call_args.write_to_out_protocol(self.o_prot_mut())?;
        self.o_prot_mut().write_message_end()?;
        self.o_prot_mut().flush()
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin()?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("getPhaseByFireflyPosition", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut())?;
        self.i_prot_mut().read_message_end()?;
        return Err(thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = FireflyServiceGetPhaseByFireflyPositionResult::read_from_in_protocol(self.i_prot_mut())?;
      self.i_prot_mut().read_message_end()?;
      result.ok_or()
    }
  }
  fn send_phase_update(&mut self, firefly: Firefly) -> thrift::Result<()> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("sendPhaseUpdate", TMessageType::Call, self.sequence_number());
        let call_args = FireflyServiceSendPhaseUpdateArgs { firefly };
        self.o_prot_mut().write_message_begin(&message_ident)?;
        call_args.write_to_out_protocol(self.o_prot_mut())?;
        self.o_prot_mut().write_message_end()?;
        self.o_prot_mut().flush()
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin()?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("sendPhaseUpdate", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut())?;
        self.i_prot_mut().read_message_end()?;
        return Err(thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = FireflyServiceSendPhaseUpdateResult::read_from_in_protocol(self.i_prot_mut())?;
      self.i_prot_mut().read_message_end()?;
      result.ok_or()
    }
  }
}

//
// FireflyService service processor
//

pub trait FireflyServiceSyncHandler {
  fn handle_get_phase_by_firefly_position(&self, position: Position) -> thrift::Result<OrderedFloat<f64>>;
  fn handle_send_phase_update(&self, firefly: Firefly) -> thrift::Result<()>;
}

pub struct FireflyServiceSyncProcessor<H: FireflyServiceSyncHandler> {
  handler: H,
}

impl <H: FireflyServiceSyncHandler> FireflyServiceSyncProcessor<H> {
  pub fn new(handler: H) -> FireflyServiceSyncProcessor<H> {
    FireflyServiceSyncProcessor {
      handler,
    }
  }
  fn process_get_phase_by_firefly_position(&self, incoming_sequence_number: i32, i_prot: &mut dyn TInputProtocol, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    TFireflyServiceProcessFunctions::process_get_phase_by_firefly_position(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
  fn process_send_phase_update(&self, incoming_sequence_number: i32, i_prot: &mut dyn TInputProtocol, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    TFireflyServiceProcessFunctions::process_send_phase_update(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
}

pub struct TFireflyServiceProcessFunctions;

impl TFireflyServiceProcessFunctions {
  pub fn process_get_phase_by_firefly_position<H: FireflyServiceSyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut dyn TInputProtocol, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let args = FireflyServiceGetPhaseByFireflyPositionArgs::read_from_in_protocol(i_prot)?;
    match handler.handle_get_phase_by_firefly_position(args.position) {
      Ok(handler_return) => {
        let message_ident = TMessageIdentifier::new("getPhaseByFireflyPosition", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident)?;
        let ret = FireflyServiceGetPhaseByFireflyPositionResult { result_value: Some(handler_return) };
        ret.write_to_out_protocol(o_prot)?;
        o_prot.write_message_end()?;
        o_prot.flush()
      },
      Err(e) => {
        match e {
          thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("getPhaseByFireflyPosition", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.to_string()
              )
            };
            let message_ident = TMessageIdentifier::new("getPhaseByFireflyPosition", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
        }
      },
    }
  }
  pub fn process_send_phase_update<H: FireflyServiceSyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut dyn TInputProtocol, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let args = FireflyServiceSendPhaseUpdateArgs::read_from_in_protocol(i_prot)?;
    match handler.handle_send_phase_update(args.firefly) {
      Ok(_) => {
        let message_ident = TMessageIdentifier::new("sendPhaseUpdate", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident)?;
        let ret = FireflyServiceSendPhaseUpdateResult {  };
        ret.write_to_out_protocol(o_prot)?;
        o_prot.write_message_end()?;
        o_prot.flush()
      },
      Err(e) => {
        match e {
          thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("sendPhaseUpdate", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.to_string()
              )
            };
            let message_ident = TMessageIdentifier::new("sendPhaseUpdate", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
        }
      },
    }
  }
}

impl <H: FireflyServiceSyncHandler> TProcessor for FireflyServiceSyncProcessor<H> {
  fn process(&self, i_prot: &mut dyn TInputProtocol, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let message_ident = i_prot.read_message_begin()?;
    let res = match &*message_ident.name {
      "getPhaseByFireflyPosition" => {
        self.process_get_phase_by_firefly_position(message_ident.sequence_number, i_prot, o_prot)
      },
      "sendPhaseUpdate" => {
        self.process_send_phase_update(message_ident.sequence_number, i_prot, o_prot)
      },
      method => {
        Err(
          thrift::Error::Application(
            ApplicationError::new(
              ApplicationErrorKind::UnknownMethod,
              format!("unknown method {}", method)
            )
          )
        )
      },
    };
    thrift::server::handle_process_result(&message_ident, res, o_prot)
  }
}

//
// FireflyServiceGetPhaseByFireflyPositionArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct FireflyServiceGetPhaseByFireflyPositionArgs {
  position: Position,
}

impl FireflyServiceGetPhaseByFireflyPositionArgs {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<FireflyServiceGetPhaseByFireflyPositionArgs> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Position> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = Position::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("FireflyServiceGetPhaseByFireflyPositionArgs.position", &f_1)?;
    let ret = FireflyServiceGetPhaseByFireflyPositionArgs {
      position: f_1.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("getPhaseByFireflyPosition_args");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("position", TType::Struct, 1))?;
    self.position.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// FireflyServiceGetPhaseByFireflyPositionResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct FireflyServiceGetPhaseByFireflyPositionResult {
  result_value: Option<OrderedFloat<f64>>,
}

impl FireflyServiceGetPhaseByFireflyPositionResult {
  fn ok_or(self) -> thrift::Result<OrderedFloat<f64>> {
    if self.result_value.is_some() {
      Ok(self.result_value.unwrap())
    } else {
      Err(
        thrift::Error::Application(
          ApplicationError::new(
            ApplicationErrorKind::MissingResult,
            "no result received for FireflyServiceGetPhaseByFireflyPosition"
          )
        )
      )
    }
  }
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<FireflyServiceGetPhaseByFireflyPositionResult> {
    i_prot.read_struct_begin()?;
    let mut f_0: Option<OrderedFloat<f64>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        0 => {
          let val = OrderedFloat::from(i_prot.read_double()?);
          f_0 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = FireflyServiceGetPhaseByFireflyPositionResult {
      result_value: f_0,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("FireflyServiceGetPhaseByFireflyPositionResult");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(fld_var) = self.result_value {
      o_prot.write_field_begin(&TFieldIdentifier::new("result_value", TType::Double, 0))?;
      o_prot.write_double(fld_var.into())?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// FireflyServiceSendPhaseUpdateArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct FireflyServiceSendPhaseUpdateArgs {
  firefly: Firefly,
}

impl FireflyServiceSendPhaseUpdateArgs {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<FireflyServiceSendPhaseUpdateArgs> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Firefly> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = Firefly::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("FireflyServiceSendPhaseUpdateArgs.firefly", &f_1)?;
    let ret = FireflyServiceSendPhaseUpdateArgs {
      firefly: f_1.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("sendPhaseUpdate_args");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("firefly", TType::Struct, 1))?;
    self.firefly.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// FireflyServiceSendPhaseUpdateResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct FireflyServiceSendPhaseUpdateResult {
}

impl FireflyServiceSendPhaseUpdateResult {
  fn ok_or(self) -> thrift::Result<()> {
    Ok(())
  }
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<FireflyServiceSendPhaseUpdateResult> {
    i_prot.read_struct_begin()?;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = FireflyServiceSendPhaseUpdateResult {};
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("FireflyServiceSendPhaseUpdateResult");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

