use std::io::Write;
use std::collections::HashMap;
use once_cell::sync::OnceCell;
use super::Utils;
use crate::core::u_duration::microseconds;
use crate::core::UTimestamp;

/**
 * atom::TraceContext holds the indentation level and other context info
 * for the trace method. Before iterating over Atom objects (which may have
 * different indentation levels or other details), create an
 * atom::TraceContext and pass it to Atom::trace.
 */
#[derive(Default)]
pub struct TraceContext {
    pub members_to_go_: u8,
    pub timestamp_data_: u8,
    pub string_data_: u8,
    pub char_count_: u8,
    pub timestamp_high_: i64, 
}

impl TraceContext {
    pub fn write_indents(&mut self, out: &mut impl Write) {
        if self.members_to_go_ != 0 {
            out.write_all(b"   ").unwrap();
            self.members_to_go_ -= 1;
        }
    }
}

pub const NIL: u8 = 0x80;
pub const BOOLEAN_: u8 = 0x81; // Spell with underbar to distinguish from Windows BOOLEAN.
pub const WILDCARD: u8 = 0x82;
pub const T_WILDCARD: u8 = 0x83;
pub const I_PTR: u8 = 0x84; // internal pointer.
pub const R_PTR: u8 = 0x85; // reference pointer.
pub const VL_PTR: u8 = 0x86; // binding map value pointer.
pub const IPGM_PTR: u8 = 0x87; // r_exec internal: index of data of a tpl arg held by an ipgm.
pub const IN_OBJ_PTR: u8 = 0x88; // r_exec internal: index of data held by an input object.
pub const VALUE_PTR: u8 = 0x89; // r_exec internal: index of data held by the overlay's value array.
pub const PROD_PTR: u8 = 0x8A; // r_exec internal: index of data held by the overlay's production array.
pub const OUT_OBJ_PTR: u8 = 0x8B; // r_exec internal: index of data held by a newly produced object.
pub const D_IN_OBJ_PTR: u8 = 0x8C; // r_exec internal: index of data held by an object referenced by an input object.
pub const ASSIGN_PTR: u8 = 0x8D; // r_exec internal: index of a hlp variable and to be assigned index of an expression that produces the value.
pub const CODE_VL_PTR: u8 = 0x8E; // pointer to a value at an index in the same code array.
pub const THIS: u8 = 0x90; // this pointer.
pub const VIEW: u8 = 0x91;
pub const MKS: u8 = 0x92;
pub const VWS: u8 = 0x93;
pub const NODE: u8 = 0xA0;
pub const DEVICE: u8 = 0xA1;
pub const DEVICE_FUNCTION: u8 = 0xA2;
pub const C_PTR: u8 = 0xC0; // chain pointer.
pub const SET: u8 = 0xC1;
pub const S_SET: u8 = 0xC2; // structured set.
pub const OBJECT: u8 = 0xC3;
pub const MARKER: u8 = 0xC4;
pub const OPERATOR: u8 = 0xC5;
pub const STRING: u8 = 0xC6;
pub const TIMESTAMP: u8 = 0xC7;
pub const GROUP: u8 = 0xC8;
pub const INSTANTIATED_PROGRAM: u8 = 0xC9;
pub const INSTANTIATED_CPP_PROGRAM: u8 = 0xCA;
pub const INSTANTIATED_INPUT_LESS_PROGRAM: u8 = 0xCB;
pub const INSTANTIATED_ANTI_PROGRAM: u8 = 0xCC;
pub const COMPOSITE_STATE: u8 = 0xCD;
pub const MODEL: u8 = 0xCE;
pub const NULL_PROGRAM: u8 = 0xCF;

#[derive(Copy, Clone)]
pub struct Atom {
    pub atom_: u32,
}

#[allow(non_snake_case)]
impl Atom {
    pub fn new(a: u32) -> Self {
        Self { atom_: a }
    }

    pub fn Float(f: f32) -> Self {
        let a = f.to_bits();
        Self::new(a >> 1)
    }

    pub fn PlusInfinity() -> Self {
        Self::new(0x3FC00000)
    }

    pub fn MinusInfinity() -> Self {
        Self::new(0x7FC00000)
    }

    pub fn UndefinedFloat() -> Self {
        Self::new(0xFFFFFFF)
    }

    pub fn Nil() -> Self {
        Self::new((NIL as u32) << 24)
    }

    pub fn Boolean(value: bool) -> Self {
        Self::new(((BOOLEAN_ as u32) << 24) + (if value { 1 } else { 0 }))
    }

    pub fn UndefinedBoolean() -> Self {
        Self::new(0x81FFFFFF)
    }

    pub fn Wildcard() -> Self { Self::Wildcard_opcode(0x00) }
 
    pub fn Wildcard_opcode(opcode: u16) -> Self {
        Self::new(((WILDCARD as u32) << 24) + (((opcode as u32) & 0x0FFF) << 8))
    }

    pub fn TailWildcard() -> Self {
        Self::new((T_WILDCARD as u32) << 24)
    }

    pub fn IPointer(index: u16) -> Self {
        Self::new(((I_PTR as u32) << 24) + ((index as u32) & 0x0FFF))
    }

    pub fn VLPointer(index: u16) -> Self {
        Self::new(((VL_PTR as u32) << 24) + ((index as u32) & 0x0FFF))
    }

    pub fn RPointer(index: u16) -> Self {
        Self::new(((R_PTR as u32) << 24) + ((index as u32) & 0x0FFF))
    }

    pub fn IPGMPointer(index: u16) -> Self {
        Self::new(((IPGM_PTR as u32) << 24) + ((index as u32) & 0x0FFF))
    }

    pub fn InObjPointer(input_index: u8, index: u16) -> Self {
        Self::new(((IN_OBJ_PTR as u32) << 24) + ((input_index as u32) << 12) +
          ((index as u32) & 0x0FFF))
    }

    pub fn DInObjPointer(relative_index: u8, index: u16) -> Self {
        Self::new(((D_IN_OBJ_PTR as u32) << 24) + ((relative_index as u32) << 12) +
          ((index as u32) & 0x0FFF))
    }

    pub fn OutObjPointer(index: u16) -> Self {
        Self::new(((OUT_OBJ_PTR as u32) << 24) + ((index as u32) & 0x0FFF))
    }

    pub fn ValuePointer(index: u16) -> Self {
        Self::new(((VALUE_PTR as u32) << 24) + ((index as u32) & 0x0FFF))
    }

    pub fn ProductionPointer(index: u16) -> Self {
        Self::new(((PROD_PTR as u32) << 24) + ((index as u32) & 0x0FFF))
    }

    pub fn AssignmentPointer(variable_index: u8, index: u16) -> Self {
        Self::new(((IPGM_PTR as u32) << 24) + ((variable_index as u32) << 16) +
          ((index as u32) & 0x0FFF))
    }

    pub fn CodeVLPointer(index: u16) -> Self { Self::CodeVLPointer_cast_opcode(index, 0x0FFF) }

    pub fn CodeVLPointer_cast_opcode(index: u16, cast_opcode: u16) -> Self {
        Self::new(((CODE_VL_PTR as u32) << 24) + (((cast_opcode as u32) & 0x0FFF) << 12) +
          ((index as u32) & 0x0FFF))
    }

    pub fn This() -> Self {
        Self::new((THIS as u32) << 24)
    }

    pub fn View() -> Self {
        Self::new((VIEW as u32) << 24)
    }

    pub fn Mks() -> Self {
        Self::new((MKS as u32) << 24)
    }

    pub fn Vws() -> Self {
        Self::new((VWS as u32) << 24)
    }

    pub fn SSet(opcode: u16, element_count: u8) -> Self {
        Self::new(((S_SET as u32) << 24) + (((opcode as u32) & 0x0FFF) << 8) +
          element_count as u32)
    }

    pub fn Set(element_count: u8) -> Self {
        Self::new(((SET as u32) << 24) + element_count as u32)
    }

    pub fn CPointer(element_count: u8) -> Self {
        Self::new(((C_PTR as u32) << 24) + element_count as u32)
    }

    pub fn Object(opcode: u16, arity: u8) -> Self {
        Self::new(((OBJECT as u32) << 24) + (((opcode as u32) & 0x0FFF) << 8) + arity as u32)
    }

    pub fn Marker(opcode: u16, arity: u8) -> Self {
        Self::new(((MARKER as u32) << 24) + (((opcode as u32) & 0x0FFF) << 8) + arity as u32)
    }

    pub fn Operator(opcode: u16, arity: u8) -> Self {
        Self::new(((OPERATOR as u32) << 24) + (((opcode as u32) & 0x0FFF) << 8) + arity as u32)
    }

    pub fn Node(node_id: u8) -> Self {
        Self::new(((NODE as u32) << 24) + ((node_id as u32) << 8))
    }

    pub fn UndefinedNode() -> Self {
        Self::new(0xA0FFFFFF)
    }

    pub fn Device(node_id: u8, class_id: u8, dev_id: u8) -> Self {
        Self::new(((DEVICE as u32) << 24) + ((node_id as u32) << 16) + ((class_id as u32) << 8) +
          dev_id as u32)
    }

    pub fn UndefinedDevice() -> Self {
        Self::new(0xA1FFFFFF)
    }

    pub fn DeviceFunction(opcode: u16) -> Self {
        Self::new(((DEVICE_FUNCTION as u32) << 24) + ((opcode as u32) << 8))
    }

    pub fn UndefinedDeviceFunction() -> Self {
        Self::new(0xA2FFFFFF)
    }

    pub fn String(character_count: u8) -> Self {
        let mut blocks: u8 = character_count / 4;
        if character_count % 4 != 0 {
            blocks += 1;
        }
        Self::new(((STRING as u32) << 24) + ((blocks as u32) << 8) + character_count as u32)
    }

    pub fn UndefinedString() -> Self {
        Self::new(0xC6FFFFFF)
    }

    pub fn Timestamp() -> Self {
        Self::new((TIMESTAMP as u32) << 24)
    }

    pub fn UndefinedTimestamp() -> Self {
        Self::new(0xC7FFFFFF)
    }

    pub fn InstantiatedProgram(opcode: u16, arity: u8) -> Self {
        Self::new(((INSTANTIATED_PROGRAM as u32) << 24) + (((opcode as u32) & 0x0FFF) << 8) +
          arity as u32)
    }

    pub fn Group(opcode: u16, arity: u8) -> Self {
        Self::new(((GROUP as u32) << 24) + (((opcode as u32) & 0x0FFF) << 8) + arity as u32)
    }

    pub fn InstantiatedCPPProgram(opcode: u16, arity: u8) -> Self {
        Self::new(((INSTANTIATED_CPP_PROGRAM as u32) << 24) + (((opcode as u32) & 0x0FFF) << 8) +
          arity as u32)
    }

    pub fn InstantiatedAntiProgram(opcode: u16, arity: u8) -> Self {
        Self::new(((INSTANTIATED_ANTI_PROGRAM as u32) << 24) + (((opcode as u32) & 0x0FFF) << 8) +
          arity as u32)
    }

    pub fn InstantiatedInputLessProgram(opcode: u16, arity: u8) -> Self {
        Self::new(((INSTANTIATED_INPUT_LESS_PROGRAM as u32) << 24) +
          (((opcode as u32) & 0x0FFF) << 8) + arity as u32)
    }

    pub fn CompositeState(opcode: u16, arity: u8) -> Self {
        Self::new(((COMPOSITE_STATE as u32) << 24) + (((opcode as u32) & 0x0FFF) << 8) +
          arity as u32)
    }

    pub fn Model(opcode: u16, arity: u8) -> Self {
        Self::new(((MODEL as u32) << 24) + (((opcode as u32) & 0x0FFF) << 8) + arity as u32)
    }

    pub fn NullProgram(take_past_inputs: bool) -> Self {
        Self::new(((NULL_PROGRAM as u32) << 24) + (if take_past_inputs { 1 } else { 0 }))
    }

    /*
    // RawPointer is not used. In any case, only define it for ARCH_32. If we want to
    // define it for ARCH_64, we need to change the byte code to use two uint32 code elements.
    pub fn RawPointer(pointer) -> Self {
        Self::new(pointer)
    }
    */

    // decoders

    pub fn isUndefined(&self) -> bool {
        self.atom_ == 0xFFFFFFFF
    }

    pub fn getDescriptor(&self) -> u8 {
        (self.atom_ >> 24) as u8
    }

    pub fn isStructural(&self) -> bool {
        (self.atom_ & 0xC0000000) == 0xC0000000 || (self.atom_ & 0xD0000000) == 0xD0000000
    }

    pub fn isFloat(&self) -> bool {
        (self.atom_ >> 31) == 0
    }

    // returns true for all undefined values.
    pub fn readsAsNil(&self) -> bool {
        self.atom_ == 0x80000000 ||
        self.atom_ == 0x3FFFFFFF ||
        self.atom_ == 0x81FFFFFF ||
        self.atom_ == 0xC1000000 ||
        self.atom_ == 0xA0FFFFFF ||
        self.atom_ == 0xA1FFFFFF ||
        self.atom_ == 0xA2FFFFFF ||
        self.atom_ == 0xC6FFFFFF
    }

    pub fn asFloat(&self) -> f32 {
        let _f = self.atom_ << 1;
        f32::from_bits(_f)
    }

    pub fn asBoolean(&self) -> bool {
        if self.atom_ & 0x000000FF != 0 { true } else { false }
    }

    pub fn isBooleanTrue(&self) -> bool { self.getDescriptor() == BOOLEAN_ && self.asBoolean() }

    pub fn isBooleanFalse(&self) -> bool { self.getDescriptor() == BOOLEAN_ && !self.asBoolean() }

    // applicable to internal, view, reference, and value pointers.
    pub fn asIndex(&self) -> u16 {
        (self.atom_ & 0x00000FFF) as u16
    }

    // applicable to IN_OBJ_PTR.
    pub fn asInputIndex(&self) -> u8 {
        ((self.atom_ & 0x000FF000) >> 12) as u8
    }

    // applicable to D_IN_OBJ_PTR.
    pub fn asRelativeIndex(&self) -> u8 {
        ((self.atom_ & 0x000FF000) >> 12) as u8
    }

    pub fn asOpcode(&self) -> u16 {
        ((self.atom_ >> 8) & 0x00000FFF) as u16
    }

    // applicable to CODE_VL_PTR.
    pub fn asCastOpcode(&self) -> u16 {
        ((self.atom_ & 0x00FFF000) >> 12) as u16
    }

    // applicable to nodes and devices.
    pub fn getNodeID(&self) -> u8 {
        ((self.atom_ & 0x00FF0000) >> 16) as u8
    }

    // applicable to devices.
    pub fn getClassID(&self) -> u8 {
        ((self.atom_ & 0x0000FF00) >> 8) as u8
    }

    // applicable to devices.
    pub fn getDeviceID(&self) -> u8 {
        (self.atom_ & 0x000000FF) as u8
    }

    pub fn asAssignmentIndex(&self) -> u8 {
        ((self.atom_ & 0x00FF0000) >> 16) as u8
    }

    // arity of operators and objects/markers/structured sets, number of atoms in pointers chains,
    // number of blocks of characters in strings.
    pub fn getAtomCount(&self) -> u8 {
        match self.getDescriptor() {
            SET
            | OBJECT
            | MARKER
            | C_PTR
            | OPERATOR
            | INSTANTIATED_PROGRAM
            | INSTANTIATED_CPP_PROGRAM
            | INSTANTIATED_INPUT_LESS_PROGRAM
            | INSTANTIATED_ANTI_PROGRAM
            | COMPOSITE_STATE
            | MODEL
            | GROUP
            | S_SET => (self.atom_ & 0x000000FF) as u8,
            STRING => ((self.atom_ & 0x0000FF00) >> 8) as u8,
            TIMESTAMP => 2,
            _ => 0,
        }
    }

    // applicable to NULL_PROGRAM.
    pub fn takesPastInputs(&self) -> bool {
        if self.atom_ & 0x00000001 != 0 { true } else { false }
    }

    // asRawPointer is not used. See RawPointer above.

    pub fn trace(&self, context: &mut TraceContext, out: &mut impl Write) {
        context.write_indents(out);
        if context.timestamp_data_ != 0 {
            // Output the timestamp value now. Otherwise, it could be interpreted
            // as an op code, etc.
            context.timestamp_data_ -= 1;
            write!(out, "{}", self.atom_).unwrap();
      
            if context.timestamp_data_ == 1 {
                // Save for the next step.
                context.timestamp_high_ = self.atom_ as i64;
            }
            else {
                // Imitate utils::get_timestamp.
                let timestamp = UTimestamp::from_duration(
                    microseconds(context.timestamp_high_ << 32 | self.atom_ as i64));
                write!(out, " {}", Utils::relative_time(timestamp)).unwrap();
            }
            return;
        }
      
        match self.getDescriptor() {
            NIL => out.write_all(b"nil").unwrap(),
            BOOLEAN_ => {
                out.write_all(b"bl: ").unwrap();
                out.write_all(if self.asBoolean() { b"true" } else { b"false" }).unwrap();
            },
            WILDCARD => out.write_all(b":").unwrap(),
            T_WILDCARD => out.write_all(b"::").unwrap(),
            I_PTR => write!(out, "iptr: {}", self.asIndex()).unwrap(),
            VL_PTR => write!(out, "vlptr: {}", self.asIndex()).unwrap(),
            R_PTR => write!(out, "rptr: {}", self.asIndex()).unwrap(),
            IPGM_PTR => write!(out, "ipgm_ptr: {}", self.asIndex()).unwrap(),
            IN_OBJ_PTR => write!(
                out, "in_obj_ptr: {} {}", self.asInputIndex(), self.asIndex()).unwrap(),
            D_IN_OBJ_PTR => write!(
                out, "d_in_obj_ptr: {} {}", self.asRelativeIndex(), self.asIndex()).unwrap(),
            OUT_OBJ_PTR => write!(out, "out_obj_ptr: {}", self.asIndex()).unwrap(),
            VALUE_PTR => write!(out, "value_ptr: {}", self.asIndex()).unwrap(),
            PROD_PTR => write!(out, "prod_ptr: {}", self.asIndex()).unwrap(),
            ASSIGN_PTR => write!(
                out, "assign_ptr: {} {}", self.asAssignmentIndex(), self.asIndex()).unwrap(),
            CODE_VL_PTR => write!(out, "code_vlptr: {}", self.asIndex()).unwrap(),
            THIS => out.write_all(b"this").unwrap(),
            VIEW => out.write_all(b"view").unwrap(),
            MKS => out.write_all(b"mks").unwrap(),
            VWS => out.write_all(b"vws").unwrap(),
            NODE => write!(out, "nid: {}", self.getNodeID()).unwrap(),
            DEVICE => write!(
                out, "did: {} {} {}", self.getNodeID(), self.getClassID(),
                self.getDeviceID()).unwrap(),
            DEVICE_FUNCTION => write!(
                out, "fid: {} ({})", self.asOpcode(), get_opcode_name(self.asOpcode())).unwrap(),
            C_PTR => {
                write!(out, "cptr: {}", self.getAtomCount()).unwrap();
                context.members_to_go_ = self.getAtomCount();
            },
            SET => {
                write!(out, "set: {}", self.getAtomCount()).unwrap();
                context.members_to_go_ = self.getAtomCount();
            },
            OBJECT => {
                write!(
                    out, "obj: {} ({}) {}", self.asOpcode(), get_opcode_name(self.asOpcode()),
                    self.getAtomCount()).unwrap();
                context.members_to_go_ = self.getAtomCount();
            },
            S_SET => {
                write!(
                    out, "s_set: {} ({}) {}", self.asOpcode(), get_opcode_name(self.asOpcode()),
                    self.getAtomCount()).unwrap();
                    context.members_to_go_ = self.getAtomCount();
            },
            MARKER => {
                write!(
                    out, "mk: {} ({}) {}", self.asOpcode(), get_opcode_name(self.asOpcode()), 
                    self.getAtomCount()).unwrap();
                context.members_to_go_ = self.getAtomCount();
            },
            OPERATOR => {
                write!(
                    out, "op: {} ({}) {}", self.asOpcode(), get_opcode_name(self.asOpcode()),
                    self.getAtomCount()).unwrap();
                context.members_to_go_ = self.getAtomCount();
            },
            STRING => {
                write!(out, "st: {}", self.getAtomCount()).unwrap();
                context.members_to_go_ = self.getAtomCount();
                context.string_data_ = context.members_to_go_;
                context.char_count_ = (self.atom_ & 0x000000FF) as u8;
            },
            TIMESTAMP => {
                out.write_all(b"us").unwrap();
                context.members_to_go_ = 2;
                context.timestamp_data_ = context.members_to_go_;
            },
            GROUP => {
                write!(
                    out, "grp: {} ({}) {}", self.asOpcode(), get_opcode_name(self.asOpcode()),
                    self.getAtomCount()).unwrap();
                context.members_to_go_ = self.getAtomCount();
            },
            INSTANTIATED_PROGRAM
            | INSTANTIATED_ANTI_PROGRAM
            | INSTANTIATED_INPUT_LESS_PROGRAM => {
                write!(
                    out, "ipgm: {} ({}) {}", self.asOpcode(), get_opcode_name(self.asOpcode()),
                    self.getAtomCount()).unwrap();
                context.members_to_go_ = self.getAtomCount();
            },
            COMPOSITE_STATE => {
                write!(
                    out, "cst: {} ({}) {}", self.asOpcode(), get_opcode_name(self.asOpcode()),
                    self.getAtomCount()).unwrap();
                context.members_to_go_ = self.getAtomCount();
            },
            MODEL => {
                write!(
                    out, "mdl: {} ({}) {}", self.asOpcode(), get_opcode_name(self.asOpcode()),
                    self.getAtomCount()).unwrap();
                context.members_to_go_ = self.getAtomCount();
            },
            NULL_PROGRAM => write!(
                out, "null pgm {}", 
                if self.takesPastInputs() { "all inputs" } else { "new inputs" }).unwrap(),
            _ => {
                if context.string_data_ != 0 {      
                    context.string_data_ -= 1;
                    let mut s = String::new();
                    let content = self.atom_.to_le_bytes();
                    for i in 0..4 {
                        let have_more = context.char_count_ > 0;
                        context.char_count_ -= 1;
                        if have_more {
                            s.push(content[i] as char);
                        }
                        else {
                            break;
                        }
                    }
                    out.write_all(s.as_bytes()).unwrap();
                } else if self.isFloat() {
                    write!(out, "nb: {:.6e}", self.asFloat()).unwrap();
                } else {
                    out.write_all(b"undef").unwrap();
                }
            }
        }
    }
}

impl Default for Atom {
    fn default() -> Self {
        Atom { atom_: 0xFFFFFFFF }
    }
}

/**
 * Set the map of opcode names used by get_opcode_name.
 * \param opcode_names The map where the key is the opcode id and the value is a set of names.
 * This copies the map.
 * \return True for success, false if the opcode names have already been set.
 */
 pub fn set_opcode_names(opcode_names: &HashMap<u16, String>) -> bool {
    match OPCODE_NAMES.set(opcode_names.clone()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn get_opcode_name(opcode: u16) -> String {
    if let Some(name) = OPCODE_NAMES.get().unwrap().get(&opcode) {
        // Debug: Should return &'static String. Maybe wait for standard support for OnceCell.
        String::from(name.as_str())
    }
    else {
        String::from("unknown")
    }
}

static OPCODE_NAMES: OnceCell<HashMap<u16, String>> = OnceCell::new();
