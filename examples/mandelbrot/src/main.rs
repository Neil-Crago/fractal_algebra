use fractal_algebra::traits::FractalType;
use fractal_algebra::traits::IFS;
use fractal_algebra::traits::Mandelbrot;
use fractal_algebra::traits::mul_fractals;

fn main() {
    let m1 = Mandelbrot {
        center_re: -0.5,
        center_im: 0.0,
        zoom: 1.0,
        metadata: Default::default(),
        tags: Default::default(),
    };
    let m2 = Mandelbrot {
        center_re: 0.3,
        center_im: 0.6,
        zoom: 2.0,
        metadata: Default::default(),
        tags: Default::default(),
    };
    let ifs1 = IFS { transform_count: 4, metadata: Default::default(), tags: Default::default() };
    let ifs2 = IFS { transform_count: 3, metadata: Default::default(), tags: Default::default() };

    println!("--- Idiomatic Subtraction: A + (-B) ---");
    let subtraction_result = m1.clone().sub(&m2.clone());
    println!("{:#?}\n", subtraction_result);

    println!("--- Chained Universal Multiplication (Intersection) ---");
    // (m1 - m2) * ifs1 => ((m1 + (-m2)) * ifs1)
    let complex_expression1 = m1.clone().sub(&m2.clone());
    let bfg1: FractalType = FractalType::Mandelbrot(complex_expression1);
    let bfg2: FractalType = FractalType::IFS(ifs1.clone());

    let complex_expression = mul_fractals(&bfg1, &bfg2);
    println!("{:#?}\n", complex_expression);

    println!("--- Hybrid Multiplication System ---");
    println!("1. Using the '*' operator (CSG Intersection):");
    let intersection_result = m1.clone().mul(&m2.clone());
    println!("{:#?}\n", intersection_result);

    println!("2. Using specialized methods:");
    let transformed_mandelbrot = m1.transform_by(&m2);
    let composed_ifs = ifs1.compose_with(&ifs2);
    println!(
        "Transformed Mandelbrot Result: {:#?}",
        transformed_mandelbrot
    );
    println!("Composed IFS Result: {:#?}\n", composed_ifs);
}
