#![allow(dead_code)]

use crate::Double;

const PERMUTATION: [usize; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

const P: [usize; 512] = {
    let mut q: [usize; 512] = [0; 512];
    let mut i: usize = 0;
    while i < 512 {
        if i < 256 {
            q[i] = PERMUTATION[i];
        } else {
            q[i] = PERMUTATION[i - 256];
        }

        i = i + 1;
    }
    i = 0;
    while i < 256 {
        i = i + 1;
    }

    q
};

fn fade(t: Double) -> Double {
    return t * t * t * (t * (t * 6. - 15.) + 10.);
}

fn lerp(t: Double, a: Double, b: Double) -> Double {
    return a + t * (b - a);
}

fn grad_xyz(hash: usize, x: Double, y: Double, z: Double) -> Double {
    let h = hash & 15; // CONVERT LO 4 BITS OF HASH CODE

    let u: Double = {
        // INTO 12 GRADIENT DIRECTIONS.
        if h < 8 {
            x
        } else {
            y
        }
    };

    let v: Double = {
        if h < 4 {
            y
        } else if (h == 12) || (h == 14) {
            x
        } else {
            z
        }
    };
    // let v = h<4 ? y : h==12||h==14 ? x : z

    let result: Double = {
        if (h & 1) == 0 {
            u
        } else {
            -u
        }
    } + {
        if (h & 2) == 0 {
            v
        } else {
            -v
        }
    };
    return result;
}

fn grad_xy(hash: usize, x: Double, y: Double) -> Double {
    let h = hash & 15; // CONVERT LO 4 BITS OF HASH CODE
    let u: Double = {
        // INTO 12 GRADIENT DIRECTIONS.
        if h < 8 {
            x
        } else {
            y
        }
    };
    let v: Double = {
        if h < 4 {
            y
        } else {
            x
        }
    };

    let result: Double = {
        if (h & 1) == 0 {
            u
        } else {
            -u
        }
    } + {
        if (h & 2) == 0 {
            v
        } else {
            -v
        }
    };

    return result;
}

#[allow(non_snake_case)]
pub fn noise_xyz(x: Double, y: Double, z: Double) -> Double {
    let X: usize = (x.floor() as usize) & 255; // FIND UNIT CUBE THAT
    let Y = (y.floor() as usize) & 255; // CONTAINS POINT.
    let Z = (z.floor() as usize) & 255;
    let x = x - x.floor(); // FIND RELATIVE X,Y,Z
    let y = y - y.floor(); // OF POINT IN CUBE.
    let z = z - z.floor();

    let u = fade(x); // COMPUTE FADE CURVES
    let v = fade(y); // FOR EACH OF X,Y,Z.
    let w = fade(z);
    let A: usize = P[X] + Y; // HASH COORDINATES OF
    let AA: usize = P[A] + Z; // THE 8 CUBE CORNERS,
    let AB: usize = P[A + 1] + Z;
    let B: usize = P[X + 1] + Y;
    let BA: usize = P[B] + Z;
    let BB: usize = P[B + 1] + Z;

    return lerp(
        w,
        lerp(
            v,
            lerp(
                u,
                grad_xyz(P[AA], x, y, z), // AND ADD
                grad_xyz(P[BA], x - 1., y, z),
            ), // BLENDED
            lerp(
                u,
                grad_xyz(P[AB], x, y - 1., z), // RESULTS
                grad_xyz(P[BB], x - 1., y - 1., z),
            ),
        ), // FROM 8
        lerp(
            v,
            lerp(
                u,
                grad_xyz(P[AA + 1], x, y, z - 1.), // CORNERS
                grad_xyz(P[BA + 1], x - 1., y, z - 1.),
            ), // OF CUBE
            lerp(
                u,
                grad_xyz(P[AB + 1], x, y - 1., z - 1.),
                grad_xyz(P[BB + 1], x - 1., y - 1., z - 1.),
            ),
        ),
    );
}

#[allow(non_snake_case)]
pub fn noise_xy(x: Double, y: Double) -> Double {
    let X = (x.floor() as usize) & 255;
    let Y = (y.floor() as usize) & 255;
    let x = x - x.floor();
    let y = y - y.floor();

    let u = fade(x);
    let v = fade(y);
    let A = P[X] + Y;
    let AA = P[A];
    let AB = P[A + 1];
    let B = P[X + 1] + Y;
    let BA = P[B];
    let BB = P[B + 1];

    let x1 = lerp(u, grad_xy(P[AA], x, y), grad_xy(P[BA], x - 1., y));
    let x2 = lerp(u, grad_xy(P[AB], x, y - 1.), grad_xy(P[BB], x - 1., y - 1.));

    return lerp(v, x1, x2);
}
