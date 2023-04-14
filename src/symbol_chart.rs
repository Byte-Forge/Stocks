use std::{
    cell::{Cell, RefCell},
};

use adw::subclass::prelude::*;
use gtk::{gdk, glib, graphene, prelude::*};
use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
pub struct QuotePoint {
    pub time: DateTime<Utc>,
    pub value: f64,
}

impl QuotePoint {
    pub fn new(time: DateTime<Utc>, value: f64) -> Self {
        Self { time, value }
    }
}

mod imp {
    use glib::{ParamSpec, Properties, Value};

    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::SymbolChart)]
    pub struct SymbolChart {
        pub start: Cell<DateTime<Utc>>,
        pub end: Cell<DateTime<Utc>>,
        #[property(get, set)]
        pub hover_position: Cell<f64>,
        pub points: RefCell<Option<Vec<QuotePoint>>>,
        pub tick_id: RefCell<Option<gtk::TickCallbackId>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SymbolChart {
        const NAME: &'static str = "StocksSymbolChart";
        type Type = super::SymbolChart;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("symbolchart");
            klass.set_accessible_role(gtk::AccessibleRole::Slider);
        }
    }

    impl ObjectImpl for SymbolChart {
        fn properties() -> &'static [ParamSpec] {
          Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
          self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
          self.derived_property(id, pspec)
        }

        fn constructed(&self) {
            self.parent_constructed();
        }

        fn dispose(&self) {
        }
    }

    impl WidgetImpl for SymbolChart {
        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }

        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            (for_size, for_size, -1, -1)
           }

        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let widget = self.obj();
            let w = widget.width();
            let h = widget.height();
            if w == 0 || h == 0 {
                return;
            }

            // Our reference line
            let center_y = h as f32 / 2.0;

            // Grab the colors
            let hc = adw::StyleManager::default().is_high_contrast();

            let style_context = widget.style_context();
            let color = style_context.color();
            let empty_opacity = if hc { 0.4 } else { 0.2 };
            let hover_opacity = if hc { 0.7 } else { 0.45 };
            let line_size = 2.0;

            let grid_color = gdk::RGBA::new(
                color.red(),
                color.green(),
                color.blue(),
                color.alpha() * empty_opacity,
            );

            let is_rtl = match widget.direction() {
                gtk::TextDirection::Rtl => true,
                _ => false,
            };
            let available_width = w - line_size as i32;

            // Draw vertical grid lines
            for x in (0..w).step_by(((w - line_size as i32) / 4) as usize) {
              snapshot.append_color(
                &grid_color,
                &graphene::Rect::new(x as f32, 0.0, line_size, h as f32),
                );
            }

             // Draw horizontal grid lines
            for y in (0..h).step_by(((h - line_size as i32) / 4) as usize) {
              snapshot.append_color(
                &grid_color,
                &graphene::Rect::new(0.0, y as f32, w as f32, line_size),
                );
            }

            // Draw graph
            if let Some(ref points) = *self.points.borrow() {
                let n_points = points.len() as i32;

            }
        }
    }
}

glib::wrapper! {
    pub struct SymbolChart(ObjectSubclass<imp::SymbolChart>)
        @extends gtk::Widget,
        @implements gtk::Accessible;
}

fn ease_out_cubic(t: f64) -> f64 {
    let p = t - 1.0;
    p * p * p + 1.0
}

impl Default for SymbolChart {
    fn default() -> Self {
        glib::Object::new()
    }
}

const ANIMATION_USECS: f64 = 250_000.0;

impl SymbolChart {
    pub fn new() -> Self {
        Self::default()
    }

    fn seek_to_date(&self, time: DateTime<Utc>) {
        let width = self.width();
;
    }

    pub fn set_points(&self, points: Option<Vec<QuotePoint>>) {
        if let Some(tick_id) = self.imp().tick_id.replace(None) {
            tick_id.remove();
        }
        let enable_animations = self.settings().is_gtk_enable_animations();
        if !enable_animations {
            self.imp().points.replace(points);
            self.queue_resize();
            return;
        }

        // TODO: animate
        // self.imp().next_peaks.replace(peak_pairs);
        // self.imp().factor.set(None);
        // self.imp().first_frame_time.set(None);

        // let tick_id = self.add_tick_callback(clone!(@strong self as this => move |_, clock| {
        //     let frame_time = clock.frame_time();
        //     if let Some(first_frame_time) = this.imp().first_frame_time.get() {
        //         if frame_time < first_frame_time {
        //             warn!("Frame clock going backwards");
        //             return glib::Continue(true);
        //         }

        //         let has_peaks = match *this.imp().peaks.borrow() {
        //             Some(_) => true,
        //             None => false,
        //         };

        //         let has_next_peaks = match *this.imp().next_peaks.borrow() {
        //             Some(_) => true,
        //             None => false,
        //         };

        //         if has_peaks && has_next_peaks {
        //             // Animate the existing peaks to zero
        //             let progress = 1.0 - ((frame_time - first_frame_time) as f64 / ANIMATION_USECS);
        //             let delta = ease_out_cubic(progress);
        //             if delta < 0.0 {
        //                 this.imp().peaks.replace(None);
        //                 this.imp().factor.replace(None);
        //             } else {
        //                 this.imp().factor.replace(Some(delta));
        //                 this.queue_draw();
        //             }
        //         } else if has_peaks && !has_next_peaks {
        //             // Animate the peaks from zero
        //             let progress = (frame_time - first_frame_time) as f64 / ANIMATION_USECS;
        //             let delta = ease_out_cubic(progress);
        //             if delta > 1.0 {
        //                 // Animation complete
        //                 this.imp().factor.replace(None);
        //                 this.imp().first_frame_time.replace(None);
        //                 this.imp().tick_id.replace(None);
        //                 return glib::Continue(false);
        //             } else {
        //                 this.imp().factor.replace(Some(delta));
        //                 this.queue_draw();
        //             }
        //         } else if !has_peaks && has_next_peaks {
        //             // Swap peaks
        //             let next_peaks = this.imp().next_peaks.take();
        //             this.imp().peaks.replace(next_peaks);
        //             this.imp().factor.replace(None);
        //             this.imp().first_frame_time.replace(None);
        //         } else {
        //             // No peaks
        //             this.imp().factor.replace(None);
        //             this.imp().tick_id.replace(None);
        //             return glib::Continue(false);
        //         }
        //     } else {
        //         this.imp().first_frame_time.replace(Some(frame_time));
        //     }
        //     glib::Continue(true)
        // }));

        // self.imp().tick_id.replace(Some(tick_id));
        // self.queue_resize();
    }
}
