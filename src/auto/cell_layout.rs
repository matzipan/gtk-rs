// This file was generated by gir (0fe730d) from gir-files (db49619)
// DO NOT EDIT

use CellArea;
use CellRenderer;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct CellLayout(Object<ffi::GtkCellLayout>);

    match fn {
        get_type => || ffi::gtk_cell_layout_get_type(),
    }
}

pub trait CellLayoutExt {
    fn add_attribute<P: IsA<CellRenderer>>(&self, cell: &P, attribute: &str, column: i32);

    fn clear(&self);

    fn clear_attributes<P: IsA<CellRenderer>>(&self, cell: &P);

    fn get_area(&self) -> Option<CellArea>;

    fn get_cells(&self) -> Vec<CellRenderer>;

    fn pack_end<P: IsA<CellRenderer>>(&self, cell: &P, expand: bool);

    fn pack_start<P: IsA<CellRenderer>>(&self, cell: &P, expand: bool);

    fn reorder<P: IsA<CellRenderer>>(&self, cell: &P, position: i32);

    //fn set_attributes<P: IsA<CellRenderer>>(&self, cell: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn set_cell_data_func<'a, P: IsA<CellRenderer>, Q: Into<Option<&'a /*Unimplemented*/CellLayoutDataFunc>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cell: &P, func: Q, func_data: R, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);
}

impl<O: IsA<CellLayout>> CellLayoutExt for O {
    fn add_attribute<P: IsA<CellRenderer>>(&self, cell: &P, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_cell_layout_add_attribute(self.to_glib_none().0, cell.to_glib_none().0, attribute.to_glib_none().0, column);
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::gtk_cell_layout_clear(self.to_glib_none().0);
        }
    }

    fn clear_attributes<P: IsA<CellRenderer>>(&self, cell: &P) {
        unsafe {
            ffi::gtk_cell_layout_clear_attributes(self.to_glib_none().0, cell.to_glib_none().0);
        }
    }

    fn get_area(&self) -> Option<CellArea> {
        unsafe {
            from_glib_none(ffi::gtk_cell_layout_get_area(self.to_glib_none().0))
        }
    }

    fn get_cells(&self) -> Vec<CellRenderer> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_cell_layout_get_cells(self.to_glib_none().0))
        }
    }

    fn pack_end<P: IsA<CellRenderer>>(&self, cell: &P, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_end(self.to_glib_none().0, cell.to_glib_none().0, expand.to_glib());
        }
    }

    fn pack_start<P: IsA<CellRenderer>>(&self, cell: &P, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_start(self.to_glib_none().0, cell.to_glib_none().0, expand.to_glib());
        }
    }

    fn reorder<P: IsA<CellRenderer>>(&self, cell: &P, position: i32) {
        unsafe {
            ffi::gtk_cell_layout_reorder(self.to_glib_none().0, cell.to_glib_none().0, position);
        }
    }

    //fn set_attributes<P: IsA<CellRenderer>>(&self, cell: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_cell_layout_set_attributes() }
    //}

    //fn set_cell_data_func<'a, P: IsA<CellRenderer>, Q: Into<Option<&'a /*Unimplemented*/CellLayoutDataFunc>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cell: &P, func: Q, func_data: R, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_cell_layout_set_cell_data_func() }
    //}
}
