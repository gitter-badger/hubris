module Nat

import Unit
import Product

inductive Nat
  | Z : Nat
  | S : Nat -> Nat
end

def add (n m : Nat) : Nat :=
    Nat.rec _ m (fun (_ : Nat) (pN : Nat) => S pN) n
end

-- def below (C : (Nat -> Type)) : Nat -> Type :=
--  fun (n : Nat) =>
--    Nat.rec
--    _
--    Star
--    (fun (m : Nat) (rest : Type) => MkProd (C m) rest)
--    n
-- end

-- def main : Nat :=
--  (fun (A : Type) => A) Z
-- end*/
