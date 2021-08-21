use crate::{Instruction, Edit, Move};

const UNEXPECTED_OVERFLOW: &'static str = "unexpected integer overflow";

fn optimise_first(instructions: Vec<Instruction>) -> Vec<Instruction> {
  let mut optimised = vec![];
  for instruction in instructions {
    if let Instruction::Loop(inner_instructions) = instruction {
      let inner_optimised = optimise(inner_instructions);
      optimised.push(match inner_optimised[..] {
        /*[Instruction::Edit(Edit::Decrement(1)), Instruction::EditOther(mov, edit)]
        | [Instruction::EditOther(mov, edit), Instruction::Edit(Edit::Decrement(1))] => {
          Instruction::EditOtherLoop(mov, edit)
        }*/
        [Instruction::Edit(Edit::Decrement(1))] => Instruction::Clear,
        _ => Instruction::Loop(inner_optimised)
      });
    } else if let Some(previous) = optimised.last() {
      match (previous, instruction) {
        (&Instruction::Edit(edit1), Instruction::Edit(edit2)) => match (edit1, edit2) {
          (Edit::Increment(incr1), Edit::Increment(incr2)) => {
            *optimised.last_mut().unwrap() = Instruction::Edit(Edit::Increment(incr1.checked_add(incr2).expect(UNEXPECTED_OVERFLOW)));
          }
          (Edit::Decrement(decr1), Edit::Decrement(decr2)) => {
            *optimised.last_mut().unwrap() = Instruction::Edit(Edit::Decrement(decr1.checked_add(decr2).expect(UNEXPECTED_OVERFLOW)));
          }
          (Edit::Increment(incr), Edit::Decrement(decr))
          | (Edit::Decrement(decr), Edit::Increment(incr)) => {
            if incr > decr {
              *optimised.last_mut().unwrap() = Instruction::Edit(Edit::Increment(incr.checked_sub(decr).expect(UNEXPECTED_OVERFLOW)));
            } else if incr < decr {
              *optimised.last_mut().unwrap() = Instruction::Edit(Edit::Decrement(decr.checked_sub(incr).expect(UNEXPECTED_OVERFLOW)));
            } else {
              optimised.pop();
            }
          }
        }
        (&Instruction::Move(move1), Instruction::Move(move2)) => match (move1, move2) {
          (Move::Right(right1), Move::Right(right2)) => {
            *optimised.last_mut().unwrap() = Instruction::Move(Move::Right(right1.checked_add(right2).expect(UNEXPECTED_OVERFLOW)));
          }
          (Move::Left(left1), Move::Left(left2)) => {
            *optimised.last_mut().unwrap() = Instruction::Move(Move::Left(left1.checked_add(left2).expect(UNEXPECTED_OVERFLOW)));
          }
          (Move::Right(right), Move::Left(left))
          | (Move::Left(left), Move::Right(right)) => {
            if right > left {
              *optimised.last_mut().unwrap() = Instruction::Move(Move::Right(right.checked_sub(left).expect(UNEXPECTED_OVERFLOW)));
            } else if right < left {
              *optimised.last_mut().unwrap() = Instruction::Move(Move::Left(left.checked_sub(right).expect(UNEXPECTED_OVERFLOW)));
            } else {
              optimised.pop();
            }
          }
        }
        (_, instruction) => optimised.push(instruction)
      }
    } else {
      optimised.push(instruction);
    }
  }
  optimised
}

fn optimise_second(instructions: Vec<Instruction>) -> Vec<Instruction> {
  let mut optimised = vec![];
  for instruction in instructions {
    if let Some(previous) = optimised.last() {
      match (previous, instruction) {
        (&Instruction::Move(mov), Instruction::Edit(edit)) => {
          *optimised.last_mut().unwrap() = Instruction::MoveEdit(mov, edit);
        }
        (&Instruction::MoveEdit(mov_before, edit), Instruction::Move(mov_after)) => match (mov_before, mov_after) {
          (Move::Left(left), Move::Right(right)) => {
            if left == right {
              *optimised.last_mut().unwrap() = Instruction::EditOther(Move::Left(left), edit);
            } else {
              optimised.push(Instruction::Move(mov_after));
            }
          }
          (Move::Right(right), Move::Left(left)) => {
            if left == right {
              *optimised.last_mut().unwrap() = Instruction::EditOther(Move::Right(right), edit);
            } else {
              optimised.push(Instruction::Move(mov_after));
            }
          }
          _ => optimised.push(Instruction::Move(mov_after))
        }
        (_, instruction) => optimised.push(instruction)
      }
    } else {
      optimised.push(instruction);
    }
  }
  optimised
}

pub fn optimise(mut instructions: Vec<Instruction>) -> Vec<Instruction> {
  instructions = optimise_first(instructions);
  instructions = optimise_second(instructions);
  instructions
}