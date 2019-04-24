use types::*;
use types::Binop::*;
use types::Exp::*;
use types::Instr::*;
use types::Val::*;

pub fn compile(e: &Exp, is: &mut Vec<Instr>) -> Vec<Instr> {
    match e {
        EI32(i) => is.push(Instr::Push(Vi32(*i))),
        EBool(b) => is.push(Instr::Push(Vbool(*b))),
        EBinop(b) => {
            let mut is_lhs = compile(&b.lhs, is);
            let mut is_rhs = compile(&b.rhs, is);
            let mut is_op = match b.op.clone() {
                BPlus => is.push(Instr::Binary(BPlus)),
                BMinus => is.push(Instr::Binary(BMinus)),
                BTimes => is.push(Instr::Binary(BTimes)),
                BDivide => is.push(Instr::Binary(BDivide)),
                BEqual => is.push(Instr::Binary(BEqual)),
                BLess_than => is.push(Instr::Binary(BLess_than)),
            };
        },
        EUnop(u) => {
            let mut is_e = compile(&u.e, is);
            let mut is_op = match u.op.clone() {
                UNeg => is.push(Instr::Unary(UNeg)),
            };
        },
        ELet(l) => {
            let mut e1 = compile(&l.e1, is);
            let mut e2 = compile(&l.e2, is);
            let mut var = compile(&l.var, is);
            let mut is_op = is.push(Instr::Set);
        },
        ESeq(s) => {
            let mut e1 = compile(&s.e1, is);
            let mut e2 = compile(&s.e2, is);
            //let mut is_op = is.push(Instr::Seq(Seq_x));
        },
        _ => panic!("Compiler Error: Unexpected expression"),
    }

    is.to_vec()
}
