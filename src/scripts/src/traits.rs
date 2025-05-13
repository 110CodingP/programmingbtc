pub trait StackOps {
    fn op_checksig(&mut self) -> bool;
    fn op_dup(&mut self) -> bool;
    fn op_hash160(&mut self) -> bool;
    fn op_hash256(&mut self) -> bool; 
}