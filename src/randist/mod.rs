#[link(name = "gsl")]
#[link(name = "gslcblas")]
extern {
    pub fn gsl_ran_ugaussian_pdf(x: f64) -> f64;
    //double gsl_ran_ugaussian_pdf (const double x);
}

#[test]
fn test_gsl_ran_ugaussian_pdf() {
    let res = unsafe {
        gsl_ran_ugaussian_pdf(1f64)
    };
}
