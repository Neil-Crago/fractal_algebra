use fractal_algebra::traits::mul_fractals;
use fractal_algebra::traits::Mandelbrot;
use fractal_algebra::traits::IFS;

fn main() {
    let m1 = Mandelbrot { center_re: -0.5, center_im: 0.0, zoom: 1.0 };
    let m2 = Mandelbrot { center_re: 0.3, center_im: 0.6, zoom: 2.0 };
    let ifs1 = IFS { transform_count: 4 };
    let ifs2 = IFS { transform_count: 3 };

    println!("--- Idiomatic Subtraction: A + (-B) ---");
    let subtraction_result = m1.clone().sub(&m2.clone());
    println!("{:#?}\n", subtraction_result);

    println!("--- Chained Universal Multiplication (Intersection) ---");
    // (m1 - m2) * ifs1 => ((m1 + (-m2)) * ifs1)
    //let complex_expression = m1.clone().sub(&m2.clone()).mul(&ifs1.clone());
    let complex_expression1 = m1.clone().sub(&m2.clone());
    let boxed_fractal1: Box<dyn fractal_algebra::traits::Fractal> = Box::new(complex_expression1);
    let boxed_fractal2: Box<dyn fractal_algebra::traits::Fractal> = Box::new(ifs1.clone());
    let complex_expression = mul_fractals(&boxed_fractal1, &boxed_fractal2);
    println!("{:#?}\n", complex_expression);

    println!("--- Hybrid Multiplication System ---");
    println!("1. Using the '*' operator (CSG Intersection):");
    let intersection_result = m1.clone().mul(&m2.clone());
    println!("{:#?}\n", intersection_result);

    println!("2. Using specialized methods:");
    let transformed_mandelbrot = m1.transform_by(&m2);
    let composed_ifs = ifs1.compose_with(&ifs2);
    println!("Transformed Mandelbrot Result: {:#?}", transformed_mandelbrot);
    println!("Composed IFS Result: {:#?}\n", composed_ifs);
}
