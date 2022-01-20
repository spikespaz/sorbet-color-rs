/*
    Copyright 2022 Jacob Birkett

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

use std::hash;

pub trait ColorType {
    fn to_array(&self) -> [f64; 3];
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl Eq for Rgb {}
impl Eq for Hsv {}
impl Eq for Hsl {}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Rgb {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Hsv {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.v.to_bits().hash(state);
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Hsl {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.l.to_bits().hash(state);
    }
}

impl From<Hsv> for Rgb {
    fn from(other: Hsv) -> Self {
        //https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
        let c = other.v * other.s;
        let h1 = other.h / 60.0;
        let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
        let (r1, g1, b1) = neighboring(c, x, h1);
        let m = other.v - c;
        let (r, g, b) = (r1 + m, g1 + m, b1 + m);

        Self { r, g, b }
    }
}

impl From<Hsl> for Rgb {
    fn from(other: Hsl) -> Self {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB
        let c = (1.0 - (2.0 * other.l - 1.0).abs()) * other.s;
        let h1 = other.h / 60.0;
        let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
        let (r1, g1, b1) = neighboring(c, x, h1);
        let m = other.l - (c / 2.0);
        let (r, g, b) = (r1 + m, g1 + m, b1 + m);

        Self { r, g, b }
    }
}

impl From<Rgb> for Hsv {
    fn from(other: Rgb) -> Self {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
        let xmax = other.r.max(other.g.max(other.b));
        let xmin = other.r.min(other.g.min(other.b));
        let c = xmax - xmin;
        let mut h = match () {
            _ if c == 0.0 => 0.0,
            _ if xmax == other.r => 60.0 * ((other.g - other.b) / c),
            _ if xmax == other.g => 60.0 * ((other.b - other.r) / c + 2.0),
            _ if xmax == other.b => 60.0 * ((other.r - other.g) / c + 4.0),
            _ => panic!(),
        };
        if h < 0.0 {
            h += 360.0
        };
        let s = match () {
            _ if xmax == 0.0 => 0.0,
            _ => c / xmax,
        };

        Self { h, s, v: xmax }
    }
}

impl From<Hsl> for Hsv {
    fn from(other: Hsl) -> Self {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_HSV
        let v = other.l + other.s * other.l.min(1.0 - other.l);
        let sv = match () {
            _ if v == 0.0 => 0.0,
            _ => 2.0 * (1.0 - other.l / v),
        };

        Self {
            h: other.h,
            s: sv,
            v,
        }
    }
}

impl From<Rgb> for Hsl {
    fn from(other: Rgb) -> Self {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
        let xmax = other.r.max(other.g.max(other.b));
        let xmin = other.r.min(other.g.min(other.b));
        let c = xmax - xmin;
        let mut h = match () {
            _ if c == 0.0 => 0.0,
            _ if xmax == other.r => 60.0 * ((other.g - other.b) / c),
            _ if xmax == other.g => 60.0 * ((other.b - other.r) / c + 2.0),
            _ if xmax == other.b => 60.0 * ((other.r - other.g) / c + 4.0),
            _ => panic!(),
        };
        if h < 0.0 {
            h += 360.0
        };
        let l = (xmax + xmin) / 2.0;
        let s = match () {
            _ if l == 0.0 || l == 1.0 => 0.0,
            _ => c / (1.0 - (2.0 * xmax - c - 1.0).abs()),
        };

        Self { h, s, l }
    }
}

impl From<Hsv> for Hsl {
    fn from(other: Hsv) -> Self {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_HSL
        let l = other.v * (1.0 - (other.s / 2.0));
        let sl = match () {
            _ if l == 0.0 || l == 1.0 => 0.0,
            _ => 2.0 * (1.0 - l / other.v),
        };

        Self {
            h: other.h,
            s: sl,
            l,
        }
    }
}

impl ColorType for Rgb {
    fn to_array(&self) -> [f64; 3] {
        [self.r, self.g, self.b]
    }
}

impl ColorType for Hsv {
    fn to_array(&self) -> [f64; 3] {
        [self.h, self.s, self.v]
    }
}

impl ColorType for Hsl {
    fn to_array(&self) -> [f64; 3] {
        [self.h, self.s, self.l]
    }
}

fn neighboring(c: f64, x: f64, h1: f64) -> (f64, f64, f64) {
    match () {
        _ if (0.0..1.0).contains(&h1) => (c, x, 0.0),
        _ if (1.0..2.0).contains(&h1) => (x, c, 0.0),
        _ if (2.0..3.0).contains(&h1) => (0.0, c, x),
        _ if (3.0..4.0).contains(&h1) => (0.0, x, c),
        _ if (4.0..5.0).contains(&h1) => (x, 0.0, c),
        _ if (5.0..6.0).contains(&h1) => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    }
}
