pub struct BaseContext<'a> {
    pub base: usize,
    pub charset: &'a [char],
    pub prefix: [char; 2]
} 

pub const CONTEXT_B16: BaseContext = BaseContext {
    base: 16,
    charset: &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'],
    prefix: ['0', 'x']
};

pub const CONTEXT_B8: BaseContext = BaseContext {
    base: 8,
    charset: &['0', '1', '2', '3', '4', '5', '6', '7'],
    prefix: ['0', 'o']
};