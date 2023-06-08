extern crate nalgebra as na;
use na::Vector3;
fn main() {
    let a = na::Matrix3::new(2.0, -1.0, 0.0,
                          -1.0, 3.0, -1.0,
                          0.0, -1.0, 3.0);
    let b = na::Vector3::new(100.0, 50.0, 50.0);

    let mut xk = na::Vector3::new(1.0, 2.0, 3.0);

    let mut grad_xk = a * xk - b;
    let hess_xk = a.clone();
    let gg:f64 = grad_xk.norm();

    const EPS:f64 = 0.001;
    //let dk:Vector3<f64> = - grad_xk.clone();
    println!("{:?}", grad_xk);
    println!("{}", gg);
    println!("{:?}", grad_xk);
    println!("{}", grad_xk.norm());

    println!("start");
    println!("{:?}", xk);
    println!("{}", (xk.transpose() * a * xk)[0]/2.0 - (b.transpose()*xk)[0] );


    while grad_xk.norm() > EPS {

        let dk:Vector3<f64> = - hess_xk.try_inverse().unwrap()*grad_xk;

        xk = xk + dk;
        grad_xk = a * xk - b;
        println!("{:?}", xk);
        println!("{}", (xk.transpose() * a * xk)[0]/2.0 - (b.transpose()*xk)[0] );
    }

    println!("newton:ans");
    println!("{:?}", xk);
    println!("{}", (xk.transpose() * a * xk)[0]/2.0 - (b.transpose()*xk)[0] );

}
