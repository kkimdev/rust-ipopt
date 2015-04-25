/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![crate_name = "ipopt"]
#![crate_type = "rlib"]

extern crate libc;

mod ipopt {
    include!(concat!(env!("OUT_DIR"), "/IpStdCInterface.rs"));
}

pub enum MatrixLayout {
    RowMajor = 0,
    ColumnMajor = 1
}

pub struct Ipopt<F, G, GradF, JacG, H> where
        F: FnMut(&[ipopt::Number], bool) -> Option<ipopt::Number>,
        G: FnMut(&[ipopt::Number], bool) -> Option<&[ipopt::Number]>,
        GradF: FnMut(&[ipopt::Number], bool) -> Option<&[ipopt::Number]>,
        JacG: FnMut(&[ipopt::Number], bool) -> Option<&[ipopt::Number]>,
        H: FnMut(&[ipopt::Number], bool) -> Option<&[ipopt::Number]> {
    ipopt_problem : ipopt::IpoptProblem,
    // TODO Consider Result instead.
    f: Option<F>,
    g: Option<G>,
    grad_f: Option<GradF>,
    // TODO review below.
    jac_g: Option<JacG>,
    h: Option<H>
}

impl<F, G, GradF, JacG, H> Ipopt<F, G, GradF, JacG, H> where
        F: FnMut(&[ipopt::Number], bool) -> Option<ipopt::Number>,
        G: FnMut(&[ipopt::Number], bool) -> Option<&[ipopt::Number]>,
        GradF: FnMut(&[ipopt::Number], bool) -> Option<&[ipopt::Number]>,
        JacG: FnMut(&[ipopt::Number], bool) -> Option<&[ipopt::Number]>,
        H: FnMut(&[ipopt::Number], bool) -> Option<&[ipopt::Number]> {
    pub fn new(
            x_count: ipopt::Index,
            x_lower_bounds: &[ipopt::Number],
            x_upper_bounds: &[ipopt::Number],
            g_count: ipopt::Index,
            g_lower_bounds: Option<&[ipopt::Number]>,
            g_upper_bounds: Option<&[ipopt::Number]>,
            jacobian_nonzero_count: ipopt::Index,
            hessian_nonzero_count: ipopt::Index,
            matrix_layout: MatrixLayout,
            f: F,
            g: G,
            grad_f: Option<GradF>,
            jac_g: Option<JacG>,
            h: Option<H>,
            ) -> Result<Ipopt<F, G, GradF, JacG, H>, &'static str> {
        let problem;
        // TODO verify bound arrays are the same before & after the call since we're passing them
        //      as mutable pointers.
        unsafe {
             problem = ipopt::CreateIpoptProblem(
                x_count,
                x_lower_bounds.as_ptr() as *mut ipopt::Number,
                x_upper_bounds.as_ptr() as *mut ipopt::Number,
                g_count,
                match g_lower_bounds {
                    None => std::ptr::null_mut(),
                    Some(bounds) => bounds.as_ptr() as *mut ipopt::Number
                },
                match g_upper_bounds {
                    None => std::ptr::null_mut(),
                    Some(bounds) => bounds.as_ptr() as *mut ipopt::Number
                },
                jacobian_nonzero_count,
                hessian_nonzero_count,
                matrix_layout as ipopt::Index,
                Some(Self::f), // eval_f
                Some(Self::g), // eval_g
                Some(Self::grad), // eval_grad_f
                Some(Self::jac_g),
                Some(Self::h),
                );
            // Check condition:
            //   if (n<1 || m<0 || !x_L || !x_U || (m>0 && (!g_L || !g_U)) ||
            //       (m==0 && nele_jac != 0) || (m>0 && nele_jac < 1) || nele_hess < 0 ||
            //       !eval_f || !eval_grad_f || (m>0 && (!eval_g || !eval_jac_g))) {
            //     return NULL;
            //   }
        }
        // TODO 'problem' null check?
        if problem.is_null() {
            Err("CreateIpoptProblem failed.")
        } else {
            Ok(Ipopt{ipopt_problem : problem,
                f : None,
                g : None,
                grad_f : None,
                jac_g : None,
                h : None})
        }
    }

    extern "C" fn g(
        n: ipopt::Index,
        x: *mut ipopt::Number,
        new_x: ipopt::Bool,
        m: ipopt::Index,
        g: *mut ipopt::Number,
        user_data: ipopt::UserDataPtr) -> ipopt::Bool {
        false as ipopt::Bool
    }

    extern "C" fn jac_g(
        n: ipopt::Index,
        x: *mut ipopt::Number,
        new_x: ipopt::Bool,
        m: ipopt::Index,
        nele_jac: ipopt::Index,
        iRow: *mut ipopt::Index,
        jCol: *mut ipopt::Index,
        values: *mut ipopt::Number,
        user_data: ipopt::UserDataPtr) -> ipopt::Bool {
        false as ipopt::Bool
    }

    extern "C" fn h(
        n: ipopt::Index,
        x: *mut ipopt::Number,
        new_x: ipopt::Bool,
        obj_factor: ipopt::Number,
        m: ipopt::Index,
        lambda: *mut ipopt::Number,
        new_lambda: ipopt::Bool,
        nele_hess: ipopt::Index,
        iRow: *mut ipopt::Index,
        jCol: *mut ipopt::Index,
        values: *mut ipopt::Number,
        user_data: ipopt::UserDataPtr) -> ipopt::Bool {
        false as ipopt::Bool
    }

    extern "C" fn f(
        n: ipopt::Index,
        x: *mut ipopt::Number,
        new_x: ipopt::Bool,
        obj_value: *mut ipopt::Number,
        user_data: ipopt::UserDataPtr) -> ipopt::Bool {

        return true as ipopt::Bool;
    }

    extern "C" fn grad(
        n: ipopt::Index,
        x: *mut ipopt::Number,
        new_x: ipopt::Bool,
        grad_f: *mut ipopt::Number,
        user_data: ipopt::UserDataPtr) -> ipopt::Bool {

        return true as ipopt::Bool;
    }
}

impl<F, G, GradF, JacG, H> Drop for Ipopt<F, G, GradF, JacG, H> where
        F: FnMut(&[ipopt::Number], bool) -> Option<ipopt::Number>,
        G: FnMut(&[ipopt::Number], bool) -> Option<&[ipopt::Number]>,
        GradF: FnMut(&[ipopt::Number], bool) -> Option<&[ipopt::Number]>,
        JacG: FnMut(&[ipopt::Number], bool) -> Option<&[ipopt::Number]>,
        H: FnMut(&[ipopt::Number], bool) -> Option<&[ipopt::Number]> {
    fn drop(&mut self) {
        unsafe {
            ipopt::FreeIpoptProblem(self.ipopt_problem);
        }
    }
}
