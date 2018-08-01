// add this to lib in case we export later, but put it in main so we can use it here.
#![feature(try_from)]

mod base;
mod baseseq;
mod aminoacid;
mod protein;

#[cfg(test)]
mod tests {
    use base::Base::*;
    use baseseq::BaseSeq;
    use std::convert::TryFrom;
    
    #[test]
    fn comp() {
        use base::complement;
        let bases = [A,C,T,G];
        for b in bases.iter() {
            let bc = complement(*b);
            assert_ne!(*b, bc);
            assert_eq!(*b, complement(bc));
        }
    }

    #[test]
    fn start() {
        use aminoacid::is_start_codon;
        assert!(is_start_codon(&[A,T,G]));
        assert!(!is_start_codon(&[T,A,C]));
    }
    #[test]
    #[should_panic]
    fn start_length() {
        use aminoacid::is_start_codon;        
        assert!(is_start_codon(&[A,T,G,C]));
    }

    #[test]
    fn seq_eq() {
        let s0 = "ACTGCCGTAATG";
        let s1 = "GTAACGCTGCGTA";
        let s2 = String::from(s0) + &String::from(s1);
        assert_eq!(BaseSeq::try_from(s0).unwrap() + BaseSeq::try_from(s1).unwrap(),
                   BaseSeq::try_from(s2).unwrap());
    }

    #[test]
    fn valid_seq() {
        BaseSeq::try_from("GATTACA").expect("this should be valid");
    }
    
    #[test]
    #[should_panic]
    fn invalid_seq() {
        let bs = BaseSeq::try_from("ABCDEFG").unwrap();
    }
}
