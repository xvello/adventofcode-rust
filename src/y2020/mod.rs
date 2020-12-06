pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d05_opt;
pub mod d06;
pub mod d06_opt;

crate::generate_tests! {
    y2020,
    d01: (866436, 276650720),
    d02: (469, 267),
    d03: (198, 5140884672),
    d04: (254, 184),
    d05: (871, 640), d05_opt: (871, 640),
    d06: (6686, 3476), d06_opt: (6686, 3476),
}
