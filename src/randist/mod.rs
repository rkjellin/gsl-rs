mod bindings {
    #[link(name = "gsl")]
    #[link(name = "gslcblas")]
    extern {
        pub fn gsl_ran_ugaussian_pdf(x: f64) -> f64;
    }
}

pub mod ugaussian {
    use super::bindings;
    pub fn pdf(x: f64) -> f64 {
        unsafe { bindings::gsl_ran_ugaussian_pdf(x) }
    }
}

#[test]
fn calcpfd() {
    use randist::ugaussian;
    ugaussian::pdf(1.0f64);
}

