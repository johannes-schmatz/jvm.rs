
#[derive(Debug)]
pub enum Error {
	UnknownOpcode ( u8 ),
}

#[warn(missing_docs)]
/// An opcode of the JVM.
#[derive(Debug)]
pub enum Opcode { // 6.5
	/// Load `reference` from array.
	///
	/// # Operand Stack
	/// ```
	/// ..., arrayref: reference, index: int ->
	/// ..., value: reference
	/// ```
	///
	/// # Description
	/// The `value` in the component of the array at `index` is retrieved and pushed onto the operand stack.
	///
	/// # Run-time Exceptions:
	/// - If `arrayref` is `null`, throw a `java.lang.NullPointerException`.
	/// - If `index` is not within the bounds of the array referenced by `arrayref`, throw an `java.lang.ArrayIndexOutOfBoundsException`.
	AALoad,
	/// Store into `reference` array.
	///
	/// # Operand Stack
	/// ```
	/// ..., arrayref: reference, index: int, value: reference ->
	/// ...
	/// ```
	///
	/// # Description
	/// The `value` is stored as the component of the array at `index`.
	/// ... something about types, TODO: fill this in
	///
	/// # Run-time Exceptions
	/// - If `arrayref` is `null`, throw a `java.lang.NullPointerException`.
	/// - If `index` is not within the bounds of the array referenced by `arrayref`, throw an `java.lang.ArrayIndexOutOfBoundsException`.
	/// - If the actual type of `value` is not assignment compatible with the actual type of the components of the array, throw an
	///   `java.lang.ArrayStoreException`.
	AAStore,
	/// Push `null`.
	///
	/// # Operand Stack
	/// ```
	/// ... ->
	/// ..., null: reference
	/// ```
	///
	/// # Description
	/// Push the `null` object `reference` onto the operand stack.
	AConstNull,
	/// Load `reference` from local variable.
	///
	/// # Format
	/// ```
	/// ALoad
	/// index
	/// ```
	///
	/// # Operand Stack
	/// ```
	/// ... ->
	/// ..., objectref: reference
	/// ```
	///
	/// # Description
	/// The `index` is an unsigned byte that must be an index into the local variable array of the current frame. The local variable at `index` must contain a
	/// `reference`. The `objectref` in the local variable at `index` is pushed onto the operand stack.
	///
	/// # Notes
	/// The [Opcode::ALoad] instruction cannot be used to load a value of type `returnAddress` from a local variable onto the operand stack. This asymmetry
	/// with the [Opcode::AStore] instruction is intentional.
	ALoad { index: u8 },
	/// Load `reference` from local variable \<n\>.
	///
	/// # Operand Stack
	/// ```
	/// ... ->
	/// ..., objectref: reference
	/// ```
	///
	/// # Description
	/// The \<n\> must be an index into the local variable array of the current frame. The local variable at \<n\> must contain a `reference`. The `objectref`
	/// in the local variable at \<n\> is pushed onto the operand stack.
	///
	/// # Notes
	/// This instruction cannot be used to load a value of type `returnAddress` from a local variable onto the operand stack. This asymmetry with the
	/// corresponding `AStore` instruction is intentional.
	ALoad0,
	/// See [Opcode::ALoad0].
	ALoad1,
	/// See [Opcode::ALoad0].
	ALoad2,
	/// See [Opcode::ALoad0].
	ALoad3,
	/// Create new array of `reference`.
	///
	/// # Format
	/// ```
	/// ANewArray
	/// indexbyte1
	/// indexbyte2
	/// ```
	///
	/// # Operand Stack
	/// ```
	/// ..., count: int ->
	/// ..., arrayref: reference
	/// ```
	///
	/// # Description
	/// The `count` represents the number of components of the array to be created.
	/// The unsigned `indexbyte1` and `indexbyte2` are used to construct an index into the run-time constant pool of the current class, where the value of the
	/// index is `(indexbyte1 << 8) | indexbyte2`.
	/// The run-time constant pool item at that index must be a symbolic reference to a class, array, or interface type.
	/// The named class, array, or interface type is resolved.
	/// A new array with components of that type, of length `count`, is allocated from the garbage-collected heap,
	/// and a reference arrayref to this new array object is pushed onto the operand stack. All components of the new array are initialized to `null`,
	/// the default value for reference types.
	///
	/// # Linking Exceptions
	/// Resolving the class, array or interface type can ofc throw.
	///
	/// # Run-time Exceptions
	/// - If `count` is less than zero, throw a `java.lang.NegativeArraySizeException`.
	ANewArray { indexbyte1: u8, indexbyte2: u8 },
	/// Return `reference` from method.
	///
	/// # Operand Stack
	/// ```
	/// ..., objectref: reference ->
	/// [empty]
	/// ```
	///
	/// # Description
	/// The `objectref` must refer to an object of a type that is assignment compatible with the type represented by the return descriptor of the current method.
	/// If the current method is a synchronized method, the monitor entered or reentered on invocation of the method is updated and possibly exited as if
	/// by execution of a [Self::MonitorExit] instruction in the current thread.
	/// If no exception is thrown, `objectref` is popped from the operand stack of the current frame and pushed onto the operand stack of the frame of the
	/// invoker. Any other values on the operand stack of the current method are discarded.
	///
	/// The interpreter then reinstates the frame of the invoker and returns control to the invoker.
	///
	/// # Run-time Exceptions
	/// ... not done yet, // TODO: fill in?
	AReturn,
	/// Get length of array.
	///
	/// # Operand Stack
	/// ```
	/// ..., arrayref: reference ->
	/// ..., length: int
	/// ```
	///
	/// # Description
	/// The `length` of the array referenced by the `arrayref` is determined and pushed onto the stack.
	///
	/// # Run-time Exception
	/// - If `arrayref` is `null`, throw a `java.lang.NullPointerException`.
	ArrayLength,
	/// Store `reference` into local variable.
	///
	/// # Format
	/// ```
	/// AStore
	/// index
	/// ```
	///
	/// # Operand Stack
	/// ```
	/// ..., objectref: reference ->
	/// ...
	/// ```
	///
	/// TODO: description
	AStore { index: u8 },
	AStore0,
	/// See [Opcode::AStore0].
	AStore1,
	/// See [Opcode::AStore0].
	AStore2,
	/// See [Opcode::AStore0].
	AStore3,
	/// Throw exception or error.
	///
	/// # Operand Stack
	/// ```
	/// ..., objectref: reference ->
	/// objectref: reference
	/// ```
	///
	/// # Description
	/// ... TODO: fill in
	///
	/// # Run-time Exception
	/// - If `objectref` is `null`, throw a `java.lang.NullPointerException` instead of `objectref.
	///
	/// # Notes
	/// ... TODO: fill in
	AThrow,
	BALoad,
	BAStore,
	BIPush,
	Breakpoint,
	CALoad,
	CAStore,
	CheckCast,
	D2f,
	D2i,
	D2l,
	DAdd,
	DALoad,
	DAStore,
	DCmpG,
	DCmpL,
	DConst0,
	DConst1,
	DDiv,
	DLoad { index: u8 },
	DLoad0,
	/// See [Opcode::DLoad0].
	DLoad1,
	/// See [Opcode::DLoad0].
	DLoad2,
	/// See [Opcode::DLoad0].
	DLoad3,
	DMul,
	DNeg,
	DRem,
	DReturn,
	DStore { index: u8 },
	DStore0,
	/// See [Opcode::DStore0].
	DStore1,
	/// See [Opcode::DStore0].
	DStore2,
	/// See [Opcode::DStore0].
	DStore3,
	DSub,
	Dup,
	DupX1,
	DupX2,
	Dup2,
	Dup2X1,
	Dup2X2,
	F2d,
	F2i,
	F2l,
	FAdd,
	FALoad,
	FAStore,
	FCmpG,
	FCmpL,
	FConst0,
	FConst1,
	FConst2,
	FDiv,
	FLoad { index: u8 },
	FLoad0,
	/// See [Opcode::FLoad0].
	FLoad1,
	/// See [Opcode::FLoad0].
	FLoad2,
	/// See [Opcode::FLoad0].
	FLoad3,
	FMul,
	FNeg,
	FRem,
	FReturn,
	FStore { index: u8 },
	FStore0,
	/// See [Opcode::FStore0].
	FStore1,
	/// See [Opcode::FStore0].
	FStore2,
	/// See [Opcode::FStore0].
	FStore3,
	FSub,
	GetField,
	GetStatic,
	/// Branch always.
	///
	/// # Format
	/// ```
	/// Goto
	/// branchbyte1
	/// branchbyte2
	/// ```
	///
	/// # Operand Stack
	/// No change.
	///
	/// # Description
	/// The unsigned bytes `branchbyte1` and `branchbyte2` are used to construct a signed 16-bit `branchoffset`:
	/// ```
	/// branchoffset = (branchbyte1 << 8) | branchbyte2
	/// ```
	/// Executions proceeds at that offset from the address of the opcode of this [Opcode::Goto] instruction.
	Goto { branchbyte1: u8, branchbyte2: u8 },
	/// Branch always, wide index.
	///
	/// # Format
	/// ```
	/// GotoW
	/// branchbyte1
	/// branchbyte2
	/// branchbyte3
	/// branchbyte4
	/// ```
	///
	/// # Operand Stack
	/// No change.
	///
	/// # Description
	/// The unsigned bytes `branchbyte1`, `branchbyte2`, `branchbyte3` and `branchbyte4` are used to construct a signed 32-bit `branchoffset`:
	/// ```
	/// branchoffset = (branchbyte1 << 24) | (branchbyte2 << 16) | (branchbyte3 << 8) | branchbyte4
	/// ```
	/// Executions proceeds at that offset from the address of the opcode of this [Opcode::GotoW] instruction.
	GotoW { branchbyte1: u8, branchbyte2: u8, branchbyte3: u8, branchbyte4: u8 },
	I2b,
	I2c,
	I2d,
	I2f,
	I2l,
	I2s,
	IAdd,
	IALoad,
	IAnd,
	IAStore,
	IConstM1, IConst0, IConst1, IConst2, IConst3, IConst4, IConst5,
	IDiv,
	IfACmpEq, IfACmpNe,
	IfICmpEq, IfICmpGe, IfICmpGt, IfICmpLe, IfICmpLt, IfICmpNe,
	IfEq, IfGe, IfGt, IfLe, IfLt, IfNe,
	IfNonNull, IfNull,
	IInc,
	ILoad { index: u8 },
	ILoad0,
	/// See [Opcode::ILoad0].
	ILoad1,
	/// See [Opcode::ILoad0].
	ILoad2,
	/// See [Opcode::ILoad0].
	ILoad3,
	ImpDep1, ImpDep2,
	IMul,
	INeg,
	InstanceOf,
	InvokeDynamic,
	InvokeInterface,
	InvokeSpecial,
	InvokeStatic,
	InvokeVirtual,
	IOr,
	IRem,
	IReturn,
	IShl,
	IShr,
	IStore { index: u8 },
	IStore0,
	/// See [Opcode::IStore0].
	IStore1,
	/// See [Opcode::IStore0].
	IStore2,
	/// See [Opcode::IStore0].
	IStore3,
	ISub,
	IUShr,
	IXor,
	Jsr,
	JsrW,
	L2d,
	L2f,
	L2i,
	LAdd,
	LALoad,
	LAnd,
	LAStore,
	LCmp,
	LConst0, LConst1,
	Ldc,
	LdcW,
	Ldc2W,
	LDiv,
	LLoad { index: u8 },
	LLoad0,
	/// See [Opcode::LLoad1].
	LLoad1,
	/// See [Opcode::LLoad1].
	LLoad2,
	/// See [Opcode::LLoad1].
	LLoad3,
	LMul,
	LNeg,
	LookupSwitch,
	LOr,
	LRem,
	LReturn,
	LShl,
	LShr,
	LStore { index: u8 },
	LStore0,
	/// See [Opcode::LStore0].
	LStore1,
	/// See [Opcode::LStore0].
	LStore2,
	/// See [Opcode::LStore0].
	LStore3,
	LSub,
	LUShr,
	LXor,
	MonitorEnter,
	MonitorExit,
	/// Create new multidimensional array.
	///
	/// # Format
	/// ```
	/// MultiANewArray
	/// indexbyte1
	/// indexbyte2
	/// dimensions
	/// ```
	///
	/// # Operand Stack
	/// ```
	/// ..., count1: int, [count2: int, ...] ->
	/// ..., arrayref: reference
	/// ```
	///
	/// # Description
	/// The `dimensions` operand is an unsigned byte that must be greater than or equal to 1. Ite represents the number of dimensions of the array to be
	/// created. The operand stack must contain `dimension` values. Each such value represents the number of components in a dimension of the array to be
	/// created and must be non-negative. The `count1` is the desired length in the first dimension, `count2` in the second, etc.
	///
	/// ...; todo: fill in
	MultiANewArray,
	New,
	NewArray,
	/// Do nothing.
	///
	/// # Operand Stack
	/// No change.
	///
	/// # Description
	/// Do nothing.
	Nop,
	Pop,
	Pop2,
	PutField,
	PutStatic,
	Ret,
	Return,
	SALoad,
	SAStore,
	/// Push `short`
	///
	/// # Format
	/// ```
	/// SIPush
	/// byte1
	/// byte2
	/// ```
	///
	/// # Operand Stack
	/// ```
	/// ... ->
	/// ..., value: int
	/// ```
	///
	/// # Description
	/// The immediate unsigned `byte1` and `byte2` values are assembled into an intermediate `short` where the value of the `short` is `(byte1 << 8) | byte2`.
	/// The intermediate value is then sign-extended to an `int` value. That value is pushed onto the operand stack.
	SIPush,
	/// Swap the top two operand stack values.
	///
	/// # Operand Stack
	/// ```
	/// ..., value2, value1 ->
	/// ..., value1, value2
	/// ```
	///
	/// # Description
	/// This instruction must not be used unless `value1` and `value2` are both values of category 1 computational types.
	///
	/// # Notes
	/// The JVM does not prove an instruction implementing swap for category 2 computational types.
	Swap,
	/// Access jump table by index and jump.
	///
	/// # Format
	/// ```
	/// TableSwitch
	/// <0-3 byte pad>
	/// defaultbyte1
	/// defaultbyte2
	/// defaultbyte3
	/// defaultbyte4
	/// lowbyte1
	/// lowbyte2
	/// lowbyte3
	/// lowbyte4
	/// highbyte1
	/// highbyte2
	/// highbyte3
	/// highbyte4
	/// (
	///   byte1
	///   byte2
	///   byte3
	///   byte4
	/// )*
	/// ```
	///
	/// # Operand Stack
	/// ```
	/// ..., index: int ->
	/// ...
	/// ```
	///
	/// # Description
	/// A [Opcode::TableSwitch] is a variable-length instruction. Immediately after the opcode, between zero and three bytes must act as padding, such that
	/// `defaultbyte1` begins at an address that is a multiple of four bytes from the start of the current method (the opcode of its first instruction).
	/// Immediately after the padding are bytes constituting three signed 32-bit values: `default`, `low`, and `high`. Immediately following are bytes
	/// constituting a series of `high - low + 1` signed 32-bit offsets. The value `low` must be less than or equal to `high`. The `high - low + 1` signed
	/// 32-bit offsets are treated as a 0-based jump table. Each of these signed 32-bit values is constructed as `(byte1 << 24) | (byte2 << 16) | (byte3 << 8)
	/// | byte4`.
	///
	/// The index must be of type int and is popped from the operand stack. If `index` is less than `low` or `index` is greater than `high`, then a target
	/// address is calculated by adding `default` to the address of the opcode of this [Opcode::TableSwitch] instruction. Otherwise, the offset at position
	/// `index - low` of the jump table is extracted. The target address is calculated by adding that offset to the address of the opcode of this
	/// [Opcode::TableSwitch] instruction. Execution then continues at the target address.
	///
	/// The target address that can be calculated from each jump table offset, as well as the one that can be calculated from `default`, must be the address
	/// of an opcode of an instruction within the method that contains this [Opcode::TableSwitch] instruction.
	TableSwitch {
		defaultbyte1: u8, defaultbyte2: u8, defaultbyte3: u8, defaultbyte4: u8,
		lowbyte1: u8, lowbyte2: u8, lowbyte3: u8, lowbyte4: u8,
		highbyte1: u8, highbyte2: u8, highbyte3: u8, highbyte4: u8,
		offsets: Vec<i32>, // [high - low + 1]
	},
	WideALoad  { indexbyte1: u8, indexbyte2: u8 },
	WideAStore { indexbyte1: u8, indexbyte2: u8 },
	WideDLoad  { indexbyte1: u8, indexbyte2: u8 },
	WideDStore { indexbyte1: u8, indexbyte2: u8 },
	WideFLoad  { indexbyte1: u8, indexbyte2: u8 },
	WideFStore { indexbyte1: u8, indexbyte2: u8 },
	WideILoad  { indexbyte1: u8, indexbyte2: u8 },
	WideIStore { indexbyte1: u8, indexbyte2: u8 },
	WideLLoad  { indexbyte1: u8, indexbyte2: u8 },
	WideLStore { indexbyte1: u8, indexbyte2: u8 },
	WideRet    { indexbyte1: u8, indexbyte2: u8 },
	WideIInc   { indexbyte1: u8, indexbyte2: u8, constbyte1: u8, constbyte2: u8 },
}

impl Opcode {
	pub fn parameter_size(&self) -> usize {
		match self {
			Opcode::IConst0 => 0,
			Opcode::IConst1 => 0,
			Opcode::IConst2 => 0,
			Opcode::IConst3 => 0,
			Opcode::IConst4 => 0,
			Opcode::IAdd => 0,
			_ => todo!(),
		}
	}
}

impl TryFrom<u8> for Opcode {
	type Error = Error;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0x32 => Ok(Opcode::AALoad),
			0x53 => Ok(Opcode::AAStore),
			0x01 => Ok(Opcode::AConstNull),
			0x19 => todo!(), // Ok(Opcode::ALoad),
			0x2a => Ok(Opcode::ALoad0),
			0x2b => Ok(Opcode::ALoad1),
			0x2c => Ok(Opcode::ALoad2),
			0x2d => Ok(Opcode::ALoad3),
			0xbd => todo!(), // Ok(Opcode::ANewArray),
			0xb0 => Ok(Opcode::AReturn),
			0xbe => Ok(Opcode::ArrayLength),
			0x3a => todo!(), // Ok(Opcode::AStore),
			0x4b => Ok(Opcode::AStore0),
			0x4c => Ok(Opcode::AStore1),
			0x4d => Ok(Opcode::AStore2),
			0x4e => Ok(Opcode::AStore3),
			0xbf => Ok(Opcode::AThrow),
			0x33 => Ok(Opcode::BALoad),
			0x54 => Ok(Opcode::BAStore),
			0x10 => Ok(Opcode::BIPush),
			0xca => Ok(Opcode::Breakpoint),
			0x34 => Ok(Opcode::CALoad),
			0x55 => Ok(Opcode::CAStore),
			0xc0 => Ok(Opcode::CheckCast),
			0x90 => Ok(Opcode::D2f),
			0x8e => Ok(Opcode::D2i),
			0x8f => Ok(Opcode::D2l),
			0x63 => Ok(Opcode::DAdd),
			0x31 => Ok(Opcode::DALoad),
			0x52 => Ok(Opcode::DAStore),
			0x98 => Ok(Opcode::DCmpG),
			0x97 => Ok(Opcode::DCmpL),
			0x0e => Ok(Opcode::DConst0),
			0x0f => Ok(Opcode::DConst1),
			0x6f => Ok(Opcode::DDiv),
			0x18 => todo!(), // Ok(Opcode::DLoad),
			0x26 => Ok(Opcode::DLoad0),
			0x27 => Ok(Opcode::DLoad1),
			0x28 => Ok(Opcode::DLoad2),
			0x29 => Ok(Opcode::DLoad3),
			0x6b => Ok(Opcode::DMul),
			0x77 => Ok(Opcode::DNeg),
			0x73 => Ok(Opcode::DRem),
			0xaf => Ok(Opcode::DReturn),
			0x39 => todo!(), // Ok(Opcode::DStore),
			0x47 => Ok(Opcode::DStore0),
			0x48 => Ok(Opcode::DStore1),
			0x49 => Ok(Opcode::DStore2),
			0x4a => Ok(Opcode::DStore3),
			0x67 => Ok(Opcode::DSub),
			0x59 => Ok(Opcode::Dup),
			0x5a => Ok(Opcode::DupX1),
			0x5b => Ok(Opcode::DupX2),
			0x5c => Ok(Opcode::Dup2),
			0x5d => Ok(Opcode::Dup2X1),
			0x5e => Ok(Opcode::Dup2X2),
			0x8d => Ok(Opcode::F2d),
			0x8b => Ok(Opcode::F2i),
			0x8c => Ok(Opcode::F2l),
			0x62 => Ok(Opcode::FAdd),
			0x30 => Ok(Opcode::FALoad),
			0x51 => Ok(Opcode::FAStore),
			0x96 => Ok(Opcode::FCmpG),
			0x95 => Ok(Opcode::FCmpL),
			0x0b => Ok(Opcode::FConst0),
			0x0c => Ok(Opcode::FConst1),
			0x0d => Ok(Opcode::FConst2),
			0x6e => Ok(Opcode::FDiv),
			0x17 => todo!(), // Ok(Opcode::FLoad),
			0x22 => Ok(Opcode::FLoad0),
			0x23 => Ok(Opcode::FLoad1),
			0x24 => Ok(Opcode::FLoad2),
			0x25 => Ok(Opcode::FLoad3),
			0x6a => Ok(Opcode::FMul),
			0x76 => Ok(Opcode::FNeg),
			0x72 => Ok(Opcode::FRem),
			0xae => Ok(Opcode::FReturn),
			0x38 => todo!(), // Ok(Opcode::FStore),
			0x43 => Ok(Opcode::FStore0),
			0x44 => Ok(Opcode::FStore1),
			0x45 => Ok(Opcode::FStore2),
			0x46 => Ok(Opcode::FStore3),
			0x66 => Ok(Opcode::FSub),
			0xb4 => Ok(Opcode::GetField),
			0xb2 => Ok(Opcode::GetStatic),
			0xa7 => todo!(), // Ok(Opcode::Goto),
			0xc8 => todo!(), // Ok(Opcode::GotoW),
			0x91 => Ok(Opcode::I2b),
			0x92 => Ok(Opcode::I2c),
			0x87 => Ok(Opcode::I2d),
			0x86 => Ok(Opcode::I2f),
			0x85 => Ok(Opcode::I2l),
			0x93 => Ok(Opcode::I2s),
			0x60 => Ok(Opcode::IAdd),
			0x2e => Ok(Opcode::IALoad),
			0x7e => Ok(Opcode::IAnd),
			0x4f => Ok(Opcode::IAStore),
			0x02 => Ok(Opcode::IConstM1),
			0x03 => Ok(Opcode::IConst0),
			0x04 => Ok(Opcode::IConst1),
			0x05 => Ok(Opcode::IConst2),
			0x06 => Ok(Opcode::IConst3),
			0x07 => Ok(Opcode::IConst4),
			0x08 => Ok(Opcode::IConst5),
			0x6c => Ok(Opcode::IDiv),
			0xa5 => Ok(Opcode::IfACmpEq),
			0xa6 => Ok(Opcode::IfACmpNe),
			0x9f => Ok(Opcode::IfICmpEq),
			0xa2 => Ok(Opcode::IfICmpGe),
			0xa3 => Ok(Opcode::IfICmpGt),
			0xa4 => Ok(Opcode::IfICmpLe),
			0xa1 => Ok(Opcode::IfICmpLt),
			0xa0 => Ok(Opcode::IfICmpNe),
			0x99 => Ok(Opcode::IfEq),
			0x9c => Ok(Opcode::IfGe),
			0x9d => Ok(Opcode::IfGt),
			0x9e => Ok(Opcode::IfLe),
			0x9b => Ok(Opcode::IfLt),
			0x9a => Ok(Opcode::IfNe),
			0xc7 => Ok(Opcode::IfNonNull),
			0xc6 => Ok(Opcode::IfNull),
			0x84 => Ok(Opcode::IInc),
			0x15 => todo!(), // Ok(Opcode::ILoad),
			0x1a => Ok(Opcode::ILoad0),
			0x1b => Ok(Opcode::ILoad1),
			0x1c => Ok(Opcode::ILoad2),
			0x1d => Ok(Opcode::ILoad3),
			0xfe => Ok(Opcode::ImpDep1),
			0xff => Ok(Opcode::ImpDep2),
			0x68 => Ok(Opcode::IMul),
			0x74 => Ok(Opcode::INeg),
			0xc1 => Ok(Opcode::InstanceOf),
			0xba => Ok(Opcode::InvokeDynamic),
			0xb9 => Ok(Opcode::InvokeInterface),
			0xb7 => Ok(Opcode::InvokeSpecial),
			0xb8 => Ok(Opcode::InvokeStatic),
			0xb6 => Ok(Opcode::InvokeVirtual),
			0x80 => Ok(Opcode::IOr),
			0x70 => Ok(Opcode::IRem),
			0xac => Ok(Opcode::IReturn),
			0x78 => Ok(Opcode::IShl),
			0x7a => Ok(Opcode::IShr),
			0x36 => todo!(), // Ok(Opcode::IStore),
			0x3b => Ok(Opcode::IStore0),
			0x3c => Ok(Opcode::IStore1),
			0x3d => Ok(Opcode::IStore2),
			0x3e => Ok(Opcode::IStore3),
			0x64 => Ok(Opcode::ISub),
			0x7c => Ok(Opcode::IUShr),
			0x82 => Ok(Opcode::IXor),
			0xa8 => Ok(Opcode::Jsr),
			0xc9 => Ok(Opcode::JsrW),
			0x8a => Ok(Opcode::L2d),
			0x89 => Ok(Opcode::L2f),
			0x88 => Ok(Opcode::L2i),
			0x61 => Ok(Opcode::LAdd),
			0x2f => Ok(Opcode::LALoad),
			0x7f => Ok(Opcode::LAnd),
			0x50 => Ok(Opcode::LAStore),
			0x94 => Ok(Opcode::LCmp),
			0x09 => Ok(Opcode::LConst0),
			0x0a => Ok(Opcode::LConst1),
			0x12 => Ok(Opcode::Ldc),
			0x13 => Ok(Opcode::LdcW),
			0x14 => Ok(Opcode::Ldc2W),
			0x6d => Ok(Opcode::LDiv),
			0x16 => todo!(), // Ok(Opcode::LLoad),
			0x1e => Ok(Opcode::LLoad0),
			0x1f => Ok(Opcode::LLoad1),
			0x20 => Ok(Opcode::LLoad2),
			0x21 => Ok(Opcode::LLoad3),
			0x69 => Ok(Opcode::LMul),
			0x75 => Ok(Opcode::LNeg),
			0xab => Ok(Opcode::LookupSwitch),
			0x81 => Ok(Opcode::LOr),
			0x71 => Ok(Opcode::LRem),
			0xad => Ok(Opcode::LReturn),
			0x79 => Ok(Opcode::LShl),
			0x7b => Ok(Opcode::LShr),
			0x37 => todo!(), // Ok(Opcode::LStore),
			0x3f => Ok(Opcode::LStore0),
			0x40 => Ok(Opcode::LStore1),
			0x41 => Ok(Opcode::LStore2),
			0x42 => Ok(Opcode::LStore3),
			0x65 => Ok(Opcode::LSub),
			0x7d => Ok(Opcode::LUShr),
			0x83 => Ok(Opcode::LXor),
			0xc2 => Ok(Opcode::MonitorEnter),
			0xc3 => Ok(Opcode::MonitorExit),
			0xc5 => Ok(Opcode::MultiANewArray),
			0xbb => Ok(Opcode::New),
			0xbc => Ok(Opcode::NewArray),
			0x00 => Ok(Opcode::Nop),
			0x57 => Ok(Opcode::Pop),
			0x58 => Ok(Opcode::Pop2),
			0xb5 => Ok(Opcode::PutField),
			0xb3 => Ok(Opcode::PutStatic),
			0xa9 => Ok(Opcode::Ret),
			0xb1 => Ok(Opcode::Return),
			0x35 => Ok(Opcode::SALoad),
			0x56 => Ok(Opcode::SAStore),
			0x11 => Ok(Opcode::SIPush),
			0x5f => Ok(Opcode::Swap),
			0xaa => todo!(), // Ok(Opcode::TableSwitch),
			0xc4 => todo!(), // Ok(Opcode::Wide),
			_ => Err(Error::UnknownOpcode(value)),
		}
	}
}