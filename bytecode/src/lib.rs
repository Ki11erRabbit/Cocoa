
pub type PoolPointer = u64;
pub type SymbolPointer = u64;
pub type TypeTag = u8;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Bytecode {
    LoadConstant(PoolPointer),
    StoreConstant(PoolPointer),
    Pop,
    Dup,
    Swap,
    StoreLocal(u8),
    LoadLocal(u8),
    StoreArgument,
    Addu8,
    Addu16,
    Addu32,
    Addu64,
    Addi8,
    Addi16,
    Addi32,
    Addi64,
    Subu8,
    Subu16,
    Subu32,
    Subu64,
    Subi8,
    Subi16,
    Subi32,
    Subi64,
    Mulu8,
    Mulu16,
    Mulu32,
    Mulu64,
    Muli8,
    Muli16,
    Muli32,
    Muli64,
    Divu8,
    Divu16,
    Divu32,
    Divu64,
    Divi8,
    Divi16,
    Divi32,
    Divi64,
    Modu8,
    Modu16,
    Modu32,
    Modu64,
    Modi8,
    Modi16,
    Modi32,
    Modi64,
    Andu8,
    Andu16,
    Andu32,
    Andu64,
    Andi8,
    Andi16,
    Andi32,
    Andi64,
    Oru8,
    Oru16,
    Oru32,
    Oru64,
    Ori8,
    Ori16,
    Ori32,
    Ori64,
    Xoru8,
    Xoru16,
    Xoru32,
    Xoru64,
    Xori8,
    Xori16,
    Xori32,
    Xori64,
    Notu8,
    Notu16,
    Notu32,
    Notu64,
    Noti8,
    Noti16,
    Noti32,
    Noti64,
    Shlu8,
    Shlu16,
    Shlu32,
    Shlu64,
    Shli8,
    Shli16,
    Shli32,
    Shli64,
    Shru8,
    Shru16,
    Shru32,
    Shru64,
    Shri8,
    Shri16,
    Shri32,
    Shri64,
    Addf32,
    Addf64,
    Subf32,
    Subf64,
    Mulf32,
    Mulf64,
    Divf32,
    Divf64,
    Modf32,
    Modf64,
    Negu8,
    Negu16,
    Negu32,
    Negu64,
    Negi8,
    Negi16,
    Negi32,
    Negi64,
    Negf32,
    Negf64,
    Equalu8,
    Equalu16,
    Equalu32,
    Equalu64,
    Equali8,
    Equali16,
    Equali32,
    Equali64,
    Equalf32,
    Equalf64,
    Greateru8,
    Greateru16,
    Greateru32,
    Greateru64,
    Greateri8,
    Greateri16,
    Greateri32,
    Greateri64,
    Greaterf32,
    Greaterf64,
    Lessu8,
    Lessu16,
    Lessu32,
    Lessu64,
    Lessi8,
    Lessi16,
    Lessi32,
    Lessi64,
    Lessf32,
    Lessf64,
    Convert(TypeTag),
    BinaryConvert(TypeTag),
    Goto(i64),
    Jump,
    If(i64),
    IfNot(i64),
    IfGreater(i64),
    IfGreaterEqual(i64),
    IfLess(i64),
    IfLessEqual(i64),
    IfNull(i64),
    IfNotNull(i64),
    InvokeFunction(SymbolPointer),
    InvokeFunctionTail(SymbolPointer),
    InvokeTrait(SymbolPointer, SymbolPointer),
    InvokeTraitTail(SymbolPointer, SymbolPointer),
    Return,
    ReturnUnit,
    CreateStruct(SymbolPointer),
    CreateEnum(SymbolPointer),
    IsA(SymbolPointer),
    GetField(u64, TypeTag),
    SetField(u64, TypeTag),
    CreateArray(TypeTag),
    ArrayGet(TypeTag),
    ArraySet(TypeTag),
    Breakpoint,
    Nop,
}

impl Bytecode {
    pub fn into_instruction(self) -> u16 {
        use Bytecode::*;
        match self {
            LoadConstant(_) => 0,
            StoreConstant(_) => 1,
            Pop => 2,
            Dup => 3,
            Swap => 4,
            StoreLocal(_) => 5,
            LoadLocal(_) => 6,
            StoreArgument => 7,
            Addu8 => 8,
            Addu16 => 9,
            Addu32 => 10,
            Addu64 => 11,
            Addi8 => 12,
            Addi16 => 13,
            Addi32 => 14,
            Addi64 => 15,
            Subu8 => 16,
            Subu16 => 17,
            Subu32 => 18,
            Subu64 => 19,
            Subi8 => 20,
            Subi16 => 21,
            Subi32 => 22,
            Subi64 => 23,
            Mulu8 => 24,
            Mulu16 => 25,
            Mulu32 => 26,
            Mulu64 => 27,
            Muli8 => 28,
            Muli16 => 29,
            Muli32 => 30,
            Muli64 => 31,
            Divu8 => 32,
            Divu16 => 33,
            Divu32 => 34,
            Divu64 => 35,
            Divi8 => 36,
            Divi16 => 37,
            Divi32 => 38,
            Divi64 => 39,
            Modu8 => 40,
            Modu16 => 41,
            Modu32 => 42,
            Modu64 => 43,
            Modi8 => 44,
            Modi16 => 45,
            Modi32 => 46,
            Modi64 => 47,
            Andu8 => 48,
            Andu16 => 49,
            Andu32 => 50,
            Andu64 => 51,
            Andi8 => 52,
            Andi16 => 53,
            Andi32 => 54,
            Andi64 => 55,
            Oru8 => 56,
            Oru16 => 57,
            Oru32 => 58,
            Oru64 => 59,
            Ori8 => 60,
            Ori16 => 61,
            Ori32 => 62,
            Ori64 => 63,
            Xoru8 => 64,
            Xoru16 => 65,
            Xoru32 => 66,
            Xoru64 => 67,
            Xori8 => 68,
            Xori16 => 69,
            Xori32 => 70,
            Xori64 => 71,
            Notu8 => 72,
            Notu16 => 73,
            Notu32 => 74,
            Notu64 => 75,
            Noti8 => 76,
            Noti16 => 77,
            Noti32 => 78,
            Noti64 => 79,
            Shlu8 => 80,
            Shlu16 => 81,
            Shlu32 => 82,
            Shlu64 => 83,
            Shli8 => 84,
            Shli16 => 85,
            Shli32 => 86,
            Shli64 => 87,
            Shru8 => 88,
            Shru16 => 89,
            Shru32 => 90,
            Shru64 => 91,
            Shri8 => 92,
            Shri16 => 93,
            Shri32 => 94,
            Shri64 => 95,
            Addf32 => 96,
            Addf64 => 97,
            Subf32 => 98,
            Subf64 => 99,
            Mulf32 => 100,
            Mulf64 => 101,
            Divf32 => 102,
            Divf64 => 103,
            Modf32 => 104,
            Modf64 => 105,
            Negu8 => 106,
            Negu16 => 107,
            Negu32 => 108,
            Negu64 => 109,
            Negi8 => 110,
            Negi16 => 111,
            Negi32 => 112,
            Negi64 => 113,
            Negf32 => 114,
            Negf64 => 115,
            Equalu8 => 116,
            Equalu16 => 117,
            Equalu32 => 118,
            Equalu64 => 119,
            Equali8 => 120,
            Equali16 => 121,
            Equali32 => 122,
            Equali64 => 123,
            Equalf32 => 124,
            Equalf64 => 125,
            Greateru8 => 126,
            Greateru16 => 127,
            Greateru32 => 128,
            Greateru64 => 129,
            Greateri8 => 130,
            Greateri16 => 131,
            Greateri32 => 132,
            Greateri64 => 133,
            Greaterf32 => 134,
            Greaterf64 => 135,
            Lessu8 => 136,
            Lessu16 => 137,
            Lessu32 => 138,
            Lessu64 => 139,
            Lessi8 => 140,
            Lessi16 => 141,
            Lessi32 => 142,
            Lessi64 => 143,
            Lessf32 => 144,
            Lessf64 => 145,
            Convert(_) => 146,
            BinaryConvert(_) => 147,
            Goto(_) => 148,
            Jump => 149,
            If(_) => 150,
            IfNot(_) => 151,
            IfGreater(_) => 152,
            IfGreaterEqual(_) => 153,
            IfLess(_) => 154,
            IfLessEqual(_) => 155,
            IfNull(_) => 156,
            IfNotNull(_) => 157,
            InvokeFunction(_) => 158,
            InvokeFunctionTail(_) => 159,
            InvokeTrait(_, _) => 160,
            InvokeTraitTail(_, _) => 161,
            Return => 162,
            ReturnUnit => 163,
            CreateStruct(_) => 164,
            CreateEnum(_) => 165,
            IsA(_) => 166,
            GetField(_, _) => 167,
            SetField(_, _) => 168,
            CreateArray(_) => 169,
            ArrayGet(_) => 170,
            ArraySet(_) => 171,
            Breakpoint => 172,
            Nop => 173,
        }
    }

    pub fn from_binary(iter: &mut dyn Iterator<Item = u8>) -> Self {
        use Bytecode::*;
        let instruction = u16::from_le_bytes([iter.next().unwrap(), iter.next().unwrap()]);
        match instruction {
            0 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let pool_pointer = u64::from_le_bytes(slice);
                LoadConstant(pool_pointer)
            }
            1 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let pool_pointer = u64::from_le_bytes(slice);
                StoreConstant(pool_pointer)
            }
            2 => Pop,
            3 => Dup,
            4 => Swap,
            5 => StoreLocal(iter.next().unwrap()),
            6 => LoadLocal(iter.next().unwrap()),
            7 => StoreArgument,
            8 => Addu8,
            9 => Addu16,
            10 => Addu32,
            11 => Addu64,
            12 => Addi8,
            13 => Addi16,
            14 => Addi32,
            15 => Addi64,
            16 => Subu8,
            17 => Subu16,
            18 => Subu32,
            19 => Subu64,
            20 => Subi8,
            21 => Subi16,
            22 => Subi32,
            23 => Subi64,
            24 => Mulu8,
            25 => Mulu16,
            26 => Mulu32,
            27 => Mulu64,
            28 => Muli8,
            29 => Muli16,
            30 => Muli32,
            31 => Muli64,
            32 => Divu8,
            33 => Divu16,
            34 => Divu32,
            35 => Divu64,
            36 => Divi8,
            37 => Divi16,
            38 => Divi32,
            39 => Divi64,
            40 => Modu8,
            41 => Modu16,
            42 => Modu32,
            43 => Modu64,
            44 => Modi8,
            45 => Modi16,
            46 => Modi32,
            47 => Modi64,
            48 => Andu8,
            49 => Andu16,
            50 => Andu32,
            51 => Andu64,
            52 => Andi8,
            53 => Andi16,
            54 => Andi32,
            55 => Andi64,
            56 => Oru8,
            57 => Oru16,
            58 => Oru32,
            59 => Oru64,
            60 => Ori8,
            61 => Ori16,
            62 => Ori32,
            63 => Ori64,
            64 => Xoru8,
            65 => Xoru16,
            66 => Xoru32,
            67 => Xoru64,
            68 => Xori8,
            69 => Xori16,
            70 => Xori32,
            71 => Xori64,
            72 => Notu8,
            73 => Notu16,
            74 => Notu32,
            75 => Notu64,
            76 => Noti8,
            77 => Noti16,
            78 => Noti32,
            79 => Noti64,
            80 => Shlu8,
            81 => Shlu16,
            82 => Shlu32,
            83 => Shlu64,
            84 => Shli8,
            85 => Shli16,
            86 => Shli32,
            87 => Shli64,
            88 => Shru8,
            89 => Shru16,
            90 => Shru32,
            91 => Shru64,
            92 => Shri8,
            93 => Shri16,
            94 => Shri32,
            95 => Shri64,
            96 => Addf32,
            97 => Addf64,
            98 => Subf32,
            99 => Subf64,
            100 => Mulf32,
            101 => Mulf64,
            102 => Divf32,
            103 => Divf64,
            104 => Modf32,
            105 => Modf64,
            106 => Negu8,
            107 => Negu16,
            108 => Negu32,
            109 => Negu64,
            110 => Negi8,
            111 => Negi16,
            112 => Negi32,
            113 => Negi64,
            114 => Negf32,
            115 => Negf64,
            116 => Equalu8,
            117 => Equalu16,
            118 => Equalu32,
            119 => Equalu64,
            120 => Equali8,
            121 => Equali16,
            122 => Equali32,
            123 => Equali64,
            124 => Equalf32,
            125 => Equalf64,
            126 => Greateru8,
            127 => Greateru16,
            128 => Greateru32,
            129 => Greateru64,
            130 => Greateri8,
            131 => Greateri16,
            132 => Greateri32,
            133 => Greateri64,
            134 => Greaterf32,
            135 => Greaterf64,
            136 => Lessu8,
            137 => Lessu16,
            138 => Lessu32,
            139 => Lessu64,
            140 => Lessi8,
            141 => Lessi16,
            142 => Lessi32,
            143 => Lessi64,
            144 => Lessf32,
            145 => Lessf64,
            146 => Convert(iter.next().unwrap()),
            147 => BinaryConvert(iter.next().unwrap()),
            148 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let offset = i64::from_le_bytes(slice);
                Goto(offset)
            }
            149 => Jump,
            150 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let offset = i64::from_le_bytes(slice);
                If(offset)
            }
            151 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let offset = i64::from_le_bytes(slice);
                IfNot(offset)
            }
            152 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let offset = i64::from_le_bytes(slice);
                IfGreater(offset)
            }
            153 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let offset = i64::from_le_bytes(slice);
                IfGreaterEqual(offset)
            }
            154 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let offset = i64::from_le_bytes(slice);
                IfLess(offset)
            }
            155 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let offset = i64::from_le_bytes(slice);
                IfLessEqual(offset)
            }
            156 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let offset = i64::from_le_bytes(slice);
                IfNull(offset)
            }
            157 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let offset = i64::from_le_bytes(slice);
                IfNotNull(offset)
            }
            158 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let symbol_pointer = u64::from_le_bytes(slice);
                InvokeFunction(symbol_pointer)
            }
            159 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let symbol_pointer = u64::from_le_bytes(slice);
                InvokeFunctionTail(symbol_pointer)
            }
            160 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let symbol_pointer = u64::from_le_bytes(slice);
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let trait_pointer = u64::from_le_bytes(slice);
                InvokeTrait(symbol_pointer, trait_pointer)
            }
            161 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let symbol_pointer = u64::from_le_bytes(slice);
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let trait_pointer = u64::from_le_bytes(slice);
                InvokeTraitTail(symbol_pointer, trait_pointer)
            }
            162 => Return,
            163 => ReturnUnit,
            164 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let symbol_pointer = u64::from_le_bytes(slice);
                CreateStruct(symbol_pointer)
            }
            165 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let symbol_pointer = u64::from_le_bytes(slice);
                CreateEnum(symbol_pointer)
            }
            166 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let symbol_pointer = u64::from_le_bytes(slice);
                IsA(symbol_pointer)
            }
            167 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let offset = u64::from_le_bytes(slice);
                GetField(offset, iter.next().unwrap())
            }
            168 => {
                let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                let offset = u64::from_le_bytes(slice);
                SetField(offset, iter.next().unwrap())
            }
            169 => CreateArray(iter.next().unwrap()),
            170 => ArrayGet(iter.next().unwrap()),
            171 => ArraySet(iter.next().unwrap()),
            172 => Breakpoint,
            173 => Nop,
            _ => panic!("Invalid instruction: {}", instruction),
        }
    }
}
