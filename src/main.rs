use chess::{Board, Move};

fn main() {
    let mut b = Board::default();
    println!("{}", b);
    println!("{:?}", b);

    let bb = Board::from_fen("r1bqk2r/ppp2ppp/2n2n2/2bpP3/2Bp4/5N2/PPP2PPP/RNBQKR2 w Qkq d6 0 7");
    println!("{}", bb);
    println!("{:?}", bb);

    b.push(Move::from_uci("e2e4"));
    println!("{}", b);

    /*
    let mut a = Bitboard(0);
    println!("{}", a);

    a |= Bitboard(2);
    a |= Bitboard(4);
    println!("{}", a);
    a &= !Bitboard(2);
    println!("{}", a);
    */
}
