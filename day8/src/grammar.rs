use std::str::FromStr;
use std::string::String;
use ast::Instruction;
use ast::Operation;
use ast::Condition;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Instr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::string::String;
    use ast::Instruction;
    use ast::Operation;
    use ast::Condition;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_21_3d_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3c_3d_22(&'input str),
        Term_22_3d_3d_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22_3e_3d_22(&'input str),
        Term_22_5c_5cn_22(&'input str),
        Term_22dec_22(&'input str),
        Term_22if_22(&'input str),
        Term_22inc_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(&'input str),
        NtCondOp(Condition),
        NtId(String),
        NtInstr(Instruction),
        NtNewLine(&'input str),
        NtNum(i32),
        NtOp(Operation),
        Nt____Instr(Instruction),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 6, 0, 7, 0, 0,
        // State 2
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 3
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0,
        // State 5
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 6
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0,
        // State 8
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
        // State 10
        13, 14, 15, 16, 17, 18, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0,
        // State 12
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 13
        -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 14
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 15
        -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3,
        // State 16
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        // State 17
        -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
        // State 18
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        0,
        -13,
        -7,
        0,
        -12,
        -11,
        0,
        -10,
        0,
        0,
        0,
        -4,
        -6,
        -5,
        -3,
        -1,
        -2,
        -8,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 2, 3, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 5, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 8, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 11, 0, 0, 0, 0, 0,
        // State 10
        12, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 19, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!=""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""\\n""###,
            r###""dec""###,
            r###""if""###,
            r###""inc""###,
            r###"r#"-?[0-9]+"#"###,
            r###"r#"[a-zA-Z]+"#"###,
        ];
        __ACTION[(__state * 12)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Instr<
        'input,
    >(
        input: &'input str,
    ) -> Result<Instruction, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (2, _) if true => 0,
                (3, _) if true => 1,
                (4, _) if true => 2,
                (5, _) if true => 3,
                (6, _) if true => 4,
                (7, _) if true => 5,
                (8, _) if true => 6,
                (9, _) if true => 7,
                (10, _) if true => 8,
                (11, _) if true => 9,
                (0, _) if true => 10,
                (1, _) if true => 11,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 12 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_21_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_3c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3c_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_3d_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_3e_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5c_5cn_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22dec_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22if_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22inc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Instruction,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // CondOp = ">" => ActionFn(4);
                let __sym0 = __pop_Term_22_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCondOp(__nt), __end));
                0
            }
            2 => {
                // CondOp = ">=" => ActionFn(5);
                let __sym0 = __pop_Term_22_3e_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCondOp(__nt), __end));
                0
            }
            3 => {
                // CondOp = "==" => ActionFn(6);
                let __sym0 = __pop_Term_22_3d_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCondOp(__nt), __end));
                0
            }
            4 => {
                // CondOp = "!=" => ActionFn(7);
                let __sym0 = __pop_Term_22_21_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCondOp(__nt), __end));
                0
            }
            5 => {
                // CondOp = "<=" => ActionFn(8);
                let __sym0 = __pop_Term_22_3c_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCondOp(__nt), __end));
                0
            }
            6 => {
                // CondOp = "<" => ActionFn(9);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCondOp(__nt), __end));
                0
            }
            7 => {
                // Id = r#"[a-zA-Z]+"# => ActionFn(11);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtId(__nt), __end));
                1
            }
            8 => {
                // Instr = Id, Op, Num, "if", Id, CondOp, Num => ActionFn(1);
                let __sym6 = __pop_NtNum(__symbols);
                let __sym5 = __pop_NtCondOp(__symbols);
                let __sym4 = __pop_NtId(__symbols);
                let __sym3 = __pop_Term_22if_22(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_NtOp(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtInstr(__nt), __end));
                2
            }
            9 => {
                // NewLine = "\\n" => ActionFn(10);
                let __sym0 = __pop_Term_22_5c_5cn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNewLine(__nt), __end));
                3
            }
            10 => {
                // Num = r#"-?[0-9]+"# => ActionFn(12);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                4
            }
            11 => {
                // Op = "inc" => ActionFn(2);
                let __sym0 = __pop_Term_22inc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                5
            }
            12 => {
                // Op = "dec" => ActionFn(3);
                let __sym0 = __pop_Term_22dec_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                5
            }
            13 => {
                // __Instr = Instr => ActionFn(0);
                let __sym0 = __pop_NtInstr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 7 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_21_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_21_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5cn_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5cn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22dec_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22dec_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22if_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22if_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22inc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22inc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCondOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Condition, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCondOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInstr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInstr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNewLine<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNewLine(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Instr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Instr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Instr::parse_Instr;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use std::string::String;
    use ast::Instruction;
    use ast::Operation;
    use ast::Condition;
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:\\-)?(?u:[0-9])+",
                "^(?u:[A-Za-z])+",
                "^(?u:!=)",
                "^(?u:<)",
                "^(?u:<=)",
                "^(?u:==)",
                "^(?u:>)",
                "^(?u:>=)",
                "^(?u:\\\\n)",
                "^(?u:dec)",
                "^(?u:if)",
                "^(?u:inc)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:\\-)?(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:[A-Za-z])+").unwrap(),
                __regex::Regex::new("^(?u:!=)").unwrap(),
                __regex::Regex::new("^(?u:<)").unwrap(),
                __regex::Regex::new("^(?u:<=)").unwrap(),
                __regex::Regex::new("^(?u:==)").unwrap(),
                __regex::Regex::new("^(?u:>)").unwrap(),
                __regex::Regex::new("^(?u:>=)").unwrap(),
                __regex::Regex::new("^(?u:\\\\n)").unwrap(),
                __regex::Regex::new("^(?u:dec)").unwrap(),
                __regex::Regex::new("^(?u:if)").unwrap(),
                __regex::Regex::new("^(?u:inc)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 12 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Instruction, usize),
) -> Instruction
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, trg_reg, _): (usize, String, usize),
    (_, trg_op, _): (usize, Operation, usize),
    (_, trg_value, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, cond_left, _): (usize, String, usize),
    (_, cond_op, _): (usize, Condition, usize),
    (_, cond_right, _): (usize, i32, usize),
) -> Instruction
{
    Instruction{ target_reg: trg_reg,
                       target_op: trg_op, 
                       target_value: trg_value,
                       cond_left : cond_left,
                       cond_op : cond_op,
                       cond_right : cond_right }
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Operation
{
    Operation::Inc
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Operation
{
    Operation::Dec
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Condition
{
    Condition::GT
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Condition
{
    Condition::GE
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Condition
{
    Condition::EQ
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Condition
{
    Condition::NE
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Condition
{
    Condition::LE
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Condition
{
    Condition::LT
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> String
{
    String::from(s)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(s).unwrap()
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
