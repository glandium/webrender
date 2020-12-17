/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate euclid;
extern crate gleam;
extern crate glutin;
extern crate webrender;
extern crate winit;

#[path = "common/boilerplate.rs"]
mod boilerplate;

use crate::boilerplate::Example;
use webrender::api::*;
use webrender::render_api::*;
use webrender::api::units::*;


fn main() {
    let mut app = App {
        font_instance_key: None,
    };

    boilerplate::main_wrapper(&mut app, None);
}

struct App {
    font_instance_key: Option<FontInstanceKey>,
}

impl Example for App {
    fn render(
        &mut self,
        api: &mut RenderApi,
        builder: &mut DisplayListBuilder,
        txn: &mut Transaction,
        _: DeviceIntSize,
        pipeline_id: PipelineId,
        _document_id: DocumentId,
    ) {
        let content_bounds = LayoutRect::new(LayoutPoint::zero(), LayoutSize::new(800.0, 600.0));
        let root_space_and_clip = SpaceAndClipInfo::root_scroll(pipeline_id);
        let spatial_id = root_space_and_clip.spatial_id;

        if self.font_instance_key.is_none() {
            let font_key = api.generate_font_key();
            let font_bytes = include_bytes!("../wrench/reftests/text/FreeSans.ttf").to_vec();
            txn.add_raw_font(font_key, font_bytes, 0);

            let font_instance_key = api.generate_font_instance_key();
            txn.add_font_instance(font_instance_key, font_key, 32.0, None, None, Vec::new());

            self.font_instance_key = Some(font_instance_key);
        }

        builder.push_simple_stacking_context(
            content_bounds.origin,
            spatial_id,
            PrimitiveFlags::IS_BACKFACE_VISIBLE,
        );

        let text_bounds = LayoutRect::new(
            LayoutPoint::new(100.0, 50.0),
            LayoutSize::new(700.0, 200.0)
        );
        let glyphs = vec![
            GlyphInstance {
                index: 48,
                point: LayoutPoint::new(100.0, 100.0),
            },
            GlyphInstance {
                index: 68,
                point: LayoutPoint::new(150.0, 100.0),
            },
            GlyphInstance {
                index: 80,
                point: LayoutPoint::new(200.0, 100.0),
            },
            GlyphInstance {
                index: 82,
                point: LayoutPoint::new(250.0, 100.0),
            },
            GlyphInstance {
                index: 81,
                point: LayoutPoint::new(300.0, 100.0),
            },
            GlyphInstance {
                index: 3,
                point: LayoutPoint::new(350.0, 100.0),
            },
            GlyphInstance {
                index: 86,
                point: LayoutPoint::new(400.0, 100.0),
            },
            GlyphInstance {
                index: 79,
                point: LayoutPoint::new(450.0, 100.0),
            },
            GlyphInstance {
                index: 72,
                point: LayoutPoint::new(500.0, 100.0),
            },
            GlyphInstance {
                index: 83,
                point: LayoutPoint::new(550.0, 100.0),
            },
            GlyphInstance {
                index: 87,
                point: LayoutPoint::new(600.0, 100.0),
            },
            GlyphInstance {
                index: 17,
                point: LayoutPoint::new(650.0, 100.0),
            },
        ];

        builder.push_text(
            &CommonItemProperties::new(
                text_bounds,
                root_space_and_clip,
            ),
            text_bounds,
            &glyphs,
            self.font_instance_key.unwrap(),
            ColorF::new(0.0, 0.0, 0.0, 1.0),
            None,
        );

        builder.pop_stacking_context();
    }
}
