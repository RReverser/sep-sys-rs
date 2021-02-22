/* automatically generated by rust-bindgen 0.57.0 */

#![allow(non_camel_case_types)]

pub const SEP_TBYTE: ::std::os::raw::c_int = 11;
pub const SEP_TINT: ::std::os::raw::c_int = 31;
pub const SEP_TFLOAT: ::std::os::raw::c_int = 42;
pub const SEP_TDOUBLE: ::std::os::raw::c_int = 82;
pub const SEP_OBJ_MERGED: ::std::os::raw::c_short = 1;
pub const SEP_OBJ_TRUNC: ::std::os::raw::c_short = 2;
pub const SEP_OBJ_DOVERFLOW: ::std::os::raw::c_short = 4;
pub const SEP_OBJ_SINGU: ::std::os::raw::c_short = 8;
pub const SEP_APER_TRUNC: ::std::os::raw::c_short = 16;
pub const SEP_APER_HASMASKED: ::std::os::raw::c_short = 32;
pub const SEP_APER_ALLMASKED: ::std::os::raw::c_short = 64;
pub const SEP_APER_NONPOSITIVE: ::std::os::raw::c_short = 128;
pub const SEP_NOISE_NONE: ::std::os::raw::c_short = 0;
pub const SEP_NOISE_STDDEV: ::std::os::raw::c_short = 1;
pub const SEP_NOISE_VAR: ::std::os::raw::c_short = 2;
pub const SEP_MASK_IGNORE: ::std::os::raw::c_short = 4;
pub const SEP_THRESH_REL: ::std::os::raw::c_int = 0;
pub const SEP_THRESH_ABS: ::std::os::raw::c_int = 1;
pub const SEP_FILTER_CONV: ::std::os::raw::c_int = 0;
pub const SEP_FILTER_MATCHED: ::std::os::raw::c_int = 1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sep_image {
    pub data: *mut ::std::os::raw::c_void,
    pub noise: *mut ::std::os::raw::c_void,
    pub mask: *mut ::std::os::raw::c_void,
    pub segmap: *mut ::std::os::raw::c_void,
    pub dtype: ::std::os::raw::c_int,
    pub ndtype: ::std::os::raw::c_int,
    pub mdtype: ::std::os::raw::c_int,
    pub sdtype: ::std::os::raw::c_int,
    pub w: ::std::os::raw::c_int,
    pub h: ::std::os::raw::c_int,
    pub noiseval: f64,
    pub noise_type: ::std::os::raw::c_short,
    pub gain: f64,
    pub maskthresh: f64,
}
impl Default for sep_image {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sep_bkg {
    pub w: ::std::os::raw::c_int,
    pub h: ::std::os::raw::c_int,
    pub bw: ::std::os::raw::c_int,
    pub bh: ::std::os::raw::c_int,
    pub nx: ::std::os::raw::c_int,
    pub ny: ::std::os::raw::c_int,
    pub n: ::std::os::raw::c_int,
    pub global: f32,
    pub globalrms: f32,
    pub back: *mut f32,
    pub dback: *mut f32,
    pub sigma: *mut f32,
    pub dsigma: *mut f32,
}
impl Default for sep_bkg {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sep_catalog {
    pub nobj: ::std::os::raw::c_int,
    pub thresh: *mut f32,
    pub npix: *mut ::std::os::raw::c_int,
    pub tnpix: *mut ::std::os::raw::c_int,
    pub xmin: *mut ::std::os::raw::c_int,
    pub xmax: *mut ::std::os::raw::c_int,
    pub ymin: *mut ::std::os::raw::c_int,
    pub ymax: *mut ::std::os::raw::c_int,
    pub x: *mut f64,
    pub y: *mut f64,
    pub x2: *mut f64,
    pub y2: *mut f64,
    pub xy: *mut f64,
    pub errx2: *mut f64,
    pub erry2: *mut f64,
    pub errxy: *mut f64,
    pub a: *mut f32,
    pub b: *mut f32,
    pub theta: *mut f32,
    pub cxx: *mut f32,
    pub cyy: *mut f32,
    pub cxy: *mut f32,
    pub cflux: *mut f32,
    pub flux: *mut f32,
    pub cpeak: *mut f32,
    pub peak: *mut f32,
    pub xcpeak: *mut ::std::os::raw::c_int,
    pub ycpeak: *mut ::std::os::raw::c_int,
    pub xpeak: *mut ::std::os::raw::c_int,
    pub ypeak: *mut ::std::os::raw::c_int,
    pub flag: *mut ::std::os::raw::c_short,
    pub pix: *mut *mut ::std::os::raw::c_int,
    pub objectspix: *mut ::std::os::raw::c_int,
}
impl Default for sep_catalog {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn sep_background(
        image: *mut sep_image,
        bw: ::std::os::raw::c_int,
        bh: ::std::os::raw::c_int,
        fw: ::std::os::raw::c_int,
        fh: ::std::os::raw::c_int,
        fthresh: f64,
        bkg: *mut *mut sep_bkg,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_bkg_global(bkg: *mut sep_bkg) -> f32;
}
extern "C" {
    pub fn sep_bkg_globalrms(bkg: *mut sep_bkg) -> f32;
}
extern "C" {
    pub fn sep_bkg_pix(
        bkg: *mut sep_bkg,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn sep_bkg_line(
        bkg: *mut sep_bkg,
        y: ::std::os::raw::c_int,
        line: *mut ::std::os::raw::c_void,
        dtype: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_bkg_subline(
        bkg: *mut sep_bkg,
        y: ::std::os::raw::c_int,
        line: *mut ::std::os::raw::c_void,
        dtype: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_bkg_rmsline(
        bkg: *mut sep_bkg,
        y: ::std::os::raw::c_int,
        line: *mut ::std::os::raw::c_void,
        dtype: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_bkg_array(
        bkg: *mut sep_bkg,
        arr: *mut ::std::os::raw::c_void,
        dtype: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_bkg_subarray(
        bkg: *mut sep_bkg,
        arr: *mut ::std::os::raw::c_void,
        dtype: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_bkg_rmsarray(
        bkg: *mut sep_bkg,
        arr: *mut ::std::os::raw::c_void,
        dtype: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_bkg_free(bkg: *mut sep_bkg);
}
extern "C" {
    pub fn sep_extract(
        image: *mut sep_image,
        thresh: f32,
        thresh_type: ::std::os::raw::c_int,
        minarea: ::std::os::raw::c_int,
        conv: *mut f32,
        convw: ::std::os::raw::c_int,
        convh: ::std::os::raw::c_int,
        filter_type: ::std::os::raw::c_int,
        deblend_nthresh: ::std::os::raw::c_int,
        deblend_cont: f64,
        clean_flag: ::std::os::raw::c_int,
        clean_param: f64,
        catalog: *mut *mut sep_catalog,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_set_extract_pixstack(val: size_t);
}
extern "C" {
    pub fn sep_get_extract_pixstack() -> size_t;
}
extern "C" {
    pub fn sep_set_sub_object_limit(val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn sep_get_sub_object_limit() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_catalog_free(catalog: *mut sep_catalog);
}
extern "C" {
    pub fn sep_sum_circle(
        image: *mut sep_image,
        x: f64,
        y: f64,
        r: f64,
        id: ::std::os::raw::c_int,
        subpix: ::std::os::raw::c_int,
        inflags: ::std::os::raw::c_short,
        sum: *mut f64,
        sumerr: *mut f64,
        area: *mut f64,
        flag: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_sum_circann(
        image: *mut sep_image,
        x: f64,
        y: f64,
        rin: f64,
        rout: f64,
        id: ::std::os::raw::c_int,
        subpix: ::std::os::raw::c_int,
        inflags: ::std::os::raw::c_short,
        sum: *mut f64,
        sumerr: *mut f64,
        area: *mut f64,
        flag: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_sum_ellipse(
        image: *mut sep_image,
        x: f64,
        y: f64,
        a: f64,
        b: f64,
        theta: f64,
        r: f64,
        id: ::std::os::raw::c_int,
        subpix: ::std::os::raw::c_int,
        inflags: ::std::os::raw::c_short,
        sum: *mut f64,
        sumerr: *mut f64,
        area: *mut f64,
        flag: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_sum_ellipann(
        image: *mut sep_image,
        x: f64,
        y: f64,
        a: f64,
        b: f64,
        theta: f64,
        rin: f64,
        rout: f64,
        id: ::std::os::raw::c_int,
        subpix: ::std::os::raw::c_int,
        inflags: ::std::os::raw::c_short,
        sum: *mut f64,
        sumerr: *mut f64,
        area: *mut f64,
        flag: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_sum_circann_multi(
        im: *mut sep_image,
        x: f64,
        y: f64,
        rmax: f64,
        n: ::std::os::raw::c_int,
        id: ::std::os::raw::c_int,
        subpix: ::std::os::raw::c_int,
        inflag: ::std::os::raw::c_short,
        sum: *mut f64,
        sumvar: *mut f64,
        area: *mut f64,
        maskarea: *mut f64,
        flag: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_flux_radius(
        im: *mut sep_image,
        x: f64,
        y: f64,
        rmax: f64,
        id: ::std::os::raw::c_int,
        subpix: ::std::os::raw::c_int,
        inflag: ::std::os::raw::c_short,
        fluxtot: *mut f64,
        fluxfrac: *mut f64,
        n: ::std::os::raw::c_int,
        r: *mut f64,
        flag: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_kron_radius(
        im: *mut sep_image,
        x: f64,
        y: f64,
        cxx: f64,
        cyy: f64,
        cxy: f64,
        r: f64,
        id: ::std::os::raw::c_int,
        kronrad: *mut f64,
        flag: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_windowed(
        im: *mut sep_image,
        x: f64,
        y: f64,
        sig: f64,
        subpix: ::std::os::raw::c_int,
        inflag: ::std::os::raw::c_short,
        xout: *mut f64,
        yout: *mut f64,
        niter: *mut ::std::os::raw::c_int,
        flag: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_set_ellipse(
        arr: *mut ::std::os::raw::c_uchar,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        x: f64,
        y: f64,
        cxx: f64,
        cyy: f64,
        cxy: f64,
        r: f64,
        val: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn sep_ellipse_axes(
        cxx: f64,
        cyy: f64,
        cxy: f64,
        a: *mut f64,
        b: *mut f64,
        theta: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sep_ellipse_coeffs(
        a: f64,
        b: f64,
        theta: f64,
        cxx: *mut f64,
        cyy: *mut f64,
        cxy: *mut f64,
    );
}
extern "C" {
    pub static mut sep_version_string: *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn sep_get_errmsg(status: ::std::os::raw::c_int, errtext: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn sep_get_errdetail(errtext: *mut ::std::os::raw::c_char);
}
pub type size_t = ::std::os::raw::c_ulonglong;
