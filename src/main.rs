use plotpy::{Canvas, Plot, Text};
use clap::Parser;
use std::fs;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Pose {
    position: Point,
    heading: f32
}

#[derive(Debug)]
struct PoseWithVelocity {
    pose: Pose,
    velocity: f32
}

#[derive(Debug)]
struct StampedPoses {
    timestamp: f32,
    poses: Vec<PoseWithVelocity>
}

#[derive(Debug)]
struct HeadingWithVelocity {
    heading: f32,
    velocity: f32
}

#[derive(Debug)]
struct Scene {
    motion_sequences: Vec<StampedPoses>,
    obstacles: Vec<Vec<Point>>,
    leeway: Option<HeadingWithVelocity>
}

// ---------------------------------------------------------------------------------------
fn plot_one_frame(mut scene: Scene, frame_index: i32, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Result<(), &'static str> {
    if (scene.motion_sequences.len() as i32) == frame_index {
        return Ok(());
    }
    let mut canvas = Canvas::new();
    let mut idx = 0;
    let colors = vec![["#00ff00", "#33ff33", "#88ff88", "#aaffaa", "#eeffee"], ["#ff0000", "#ff3333", "#ff8888", "#ffaaaa", "#ffeeee"], ["#0000ff", "#3333ff", "#8888ff", "#aaaaff", "#eeeeff"], ["#ffff00", "#ffff33", "#ffff88", "#ffffaa", "#ffffee"]];
    for obstacle in &scene.obstacles {
        let mut pts = Vec::<Vec::<f64>>::new();
        for point in obstacle {
            pts.push(vec![point.x.into(), point.y.into()]);
        }
        canvas.set_line_width(3.0).set_face_color("#444444").set_edge_color("black");
        canvas.draw_polyline(&pts, true);
    }


    let mut sequence_idx = 0;
    let trail_steps = 4;
    for sequence in &scene.motion_sequences {
        idx = 0;
        for point in &sequence.poses {
            if sequence_idx < frame_index { // draw trail if sequence_idx is within trail_steps steps of frame_index
                if frame_index - sequence_idx <= trail_steps {
                    // line from past to current
                    if scene.motion_sequences.len() > sequence_idx.try_into().unwrap() {
                        canvas.set_face_color("None").set_edge_color(colors[idx][(frame_index - sequence_idx) as usize]).set_line_width(2.0);
                        canvas.set_arrow_scale(20.0).set_arrow_style("-");
                        canvas.draw_arrow(scene.motion_sequences[sequence_idx as usize].poses[idx].pose.position.x, scene.motion_sequences[sequence_idx as usize].poses[idx].pose.position.y, scene.motion_sequences[(sequence_idx + 1) as usize].poses[idx].pose.position.x, scene.motion_sequences[(sequence_idx + 1) as usize].poses[idx].pose.position.y);
                    }
                }
            }
            if sequence_idx == frame_index {
                canvas.set_face_color("None").set_edge_color(colors[idx][0]).set_line_width(2.0);
                canvas.draw_circle(point.pose.position.x, point.pose.position.y, 0.1);
            }
            idx += 1;
        }
        sequence_idx += 1;
    }
    // Timestamp
    let mut timestamp_text = Text::new();
    let plate_offset_x = 16.2;
    let plate_offset_y = 0.0;
    let plate_offset_text = 0.2;
    let plate_padding = 0.1;
    let plate_width = 4.0;
    let plate_height = 1.0;
    let plate = &[[min_x + plate_offset_x - plate_padding, min_y + plate_offset_y - plate_padding], [min_x + plate_offset_x - plate_padding + plate_width, min_y + plate_offset_y - plate_padding], [min_x + plate_offset_x - plate_padding + plate_width, min_y + plate_offset_y + plate_padding + plate_height], [min_x + plate_offset_x - plate_padding, min_y + plate_offset_y + plate_padding + plate_height]];
    canvas.set_line_width(3.0).set_edge_color("black").set_face_color("#ffffff").set_line_width(0.5);
    canvas.draw_polyline(plate, true);
    timestamp_text.set_color("black")
    .set_align_horizontal("left")
    .set_align_vertical("center")
    .set_fontsize(16.0)
    .set_rotation(0.0)
    .set_bbox(false)
    .set_bbox_facecolor("pink")
    .set_bbox_edgecolor("black")
    .set_bbox_alpha(0.3)
    .set_bbox_style("roundtooth,pad=0.3,tooth_size=0.2");
    let together = format!("t = {}{}", &(scene.motion_sequences[frame_index as usize].timestamp.to_string()), &" min");
    timestamp_text.draw((min_x + plate_offset_x + plate_offset_text).into(), (min_y + plate_offset_y + plate_height / 2.0).into(), &together);
    canvas.set_face_color("None").set_edge_color("black").set_line_width(2.0);
    


    let mut text = Text::new();
    if scene.leeway.is_some() {
        let length = 0.6;
        let plate_offset_x = 1.0;
        let plate_offset_y = 1.0;
        let plate_offset_text = 0.3;
        let plate_padding = 0.1;
        let plate_width = 4.0;
        // into polar coordinates
        let x_offset = length * ((scene.leeway.as_ref().unwrap().heading / 180.0) * 3.14159).sin();
        let y_offset = length * ((scene.leeway.as_ref().unwrap().heading / 180.0) * 3.14159).cos();
        // plate
        let plate = &[[min_x + plate_offset_x - length - plate_padding, min_y + plate_offset_y - length - plate_padding], [min_x + plate_offset_x - length - plate_padding + plate_width, min_y + plate_offset_y - length - plate_padding], [min_x + plate_offset_x - length - plate_padding + plate_width, min_y + plate_offset_y + plate_padding + length], [min_x + plate_offset_x - length - plate_padding, min_y + plate_offset_y + plate_padding + length]];
        canvas.set_edge_color("black").set_face_color("#eeeeee").set_line_width(0.5);
        canvas.draw_polyline(plate, true);
        // draw arrow
        canvas.set_arrow_scale(20.0).set_arrow_style("->");
        canvas.draw_arrow(min_x + plate_offset_x, min_y + plate_offset_y, min_x + plate_offset_x + x_offset, min_y + plate_offset_y + y_offset);

        text.set_color("black")
        .set_align_horizontal("left")
        .set_align_vertical("center")
        .set_fontsize(16.0)
        .set_rotation(0.0)
        .set_bbox(false)
        .set_bbox_facecolor("pink")
        .set_bbox_edgecolor("black")
        .set_bbox_alpha(0.3)
        .set_bbox_style("roundtooth,pad=0.3,tooth_size=0.2");
        let together = format!("{}{}", &(scene.leeway.as_ref().unwrap().velocity.to_string()), &" kts");
        text.draw((min_x + plate_offset_x + length + plate_offset_text).into(), (min_y + plate_offset_y).into(), &together);
        canvas.set_face_color("None").set_edge_color("black").set_line_width(2.0);
        canvas.draw_circle(min_x + plate_offset_x, min_y + plate_offset_y, length);
    }

    // add canvas to plot
    let mut plot = Plot::new();
    let padding = 0.2;
    plot.set_hide_axes(false)
        .set_figure_size_points(800.0, 800.0)
        .set_equal_axes(true)
        .set_range((min_x - padding).into(), (max_x + padding).into(), (min_y - padding).into(), (max_y + padding).into())
        .add(&canvas).add(&text).add(&timestamp_text);

    // save figure
    let filename = format!("result{}.svg", frame_index);
    println!("Writing file {}...", filename);
    plot.save(&filename)?;
    return plot_one_frame(scene, frame_index + 1, min_x, min_y, max_x, max_y);
    //Ok(())
}

fn plot_scene(mut scene: Scene) -> Result<(), &'static str> {
    // find min max
    let mut min_x = 0.0;
    let mut min_y = 0.0;
    let mut max_x = 0.0;
    let mut max_y = 0.0;
    if scene.motion_sequences.len() > 0 {
        if scene.motion_sequences[0].poses.len() > 0 {
            min_x = scene.motion_sequences[0].poses[0].pose.position.x;
            min_y = scene.motion_sequences[0].poses[0].pose.position.y;
            max_x = scene.motion_sequences[0].poses[0].pose.position.x;
            max_y = scene.motion_sequences[0].poses[0].pose.position.y;
        }
    }
    for sequence in &scene.motion_sequences {
        for point in &sequence.poses {
            if point.pose.position.x > max_x {
                max_x = point.pose.position.x;
            }
            if point.pose.position.y > max_y {
                max_y = point.pose.position.y;
            }
            if point.pose.position.x < min_x {
                min_x = point.pose.position.x;
            }
            if point.pose.position.y < min_y {
                min_y = point.pose.position.y;
            }
        }
    }
    println!("Scene bounds [{}, {}, {}, {}]", min_x, min_y, max_x, max_y);

    return plot_one_frame(scene, 0, min_x, min_y, max_x, max_y);
}

fn get_scene_csv(filename: String) -> Scene {
    let mut result = Scene {
        motion_sequences: Vec::new(),
        obstacles: Vec::new(),
        leeway: None
    };
    let mut obstacle_count = 0;
    let mut sequence_count = 0;
    let contents = fs::read_to_string(filename)
    .expect("Should have been able to read the file");
    let mut split = contents.split("\n");
    for s in split {
        let elements: Vec<&str> = s.split(";").collect();
        if (elements[0] == "SP") {
            result.motion_sequences.push(StampedPoses { timestamp: elements[1].parse().unwrap(), poses: Vec::new()});
            for i in (2..elements.len()).step_by(4) {
                if elements[i].len() > 0 {
                    result.motion_sequences[sequence_count].poses.push(PoseWithVelocity { pose: Pose { position: Point { x: elements[i].parse().unwrap(), y: elements[i+1].parse().unwrap() }, heading: elements[i+2].parse().unwrap() }, velocity: elements[i+3].parse().unwrap() });
                }
            }
            sequence_count += 1;

        } else if (elements[0] == "LW") {
            result.leeway = Some(HeadingWithVelocity { heading: elements[1].parse().unwrap(), velocity: elements[2].parse().unwrap() });
        } else if (elements[0] == "OBS") {
            result.obstacles.push(Vec::new());
            for i in (1..elements.len()).step_by(2) {
                if elements[i].len() > 0 {
                    result.obstacles[obstacle_count].push(Point { x: elements[i].parse().unwrap(), y: elements[i+1].parse().unwrap()});
                }
            }
            obstacle_count += 1;
        }
    }
    result
}

fn get_scene_json(filename: String) -> Scene {
    let mut result = Scene {
        motion_sequences: Vec::new(),
        obstacles: Vec::new(),
        leeway: None
    };
    let json = json::parse(&fs::read_to_string(filename).expect("Should have been able to read the file")).unwrap();
    let mut obstacle_count = 0;
    let mut sequence_count = 0;
    for value in json["motion_sequences"].members() {
        result.motion_sequences.push(StampedPoses { timestamp: value["timestamp"].as_f32().unwrap(), poses: Vec::new()});
        for pose in value["poses"].members() {
            result.motion_sequences[sequence_count].poses.push(PoseWithVelocity { pose: Pose { position: Point { x: pose["pose"]["position"]["x"].as_f32().unwrap(), y: pose["pose"]["position"]["y"].as_f32().unwrap() }, heading: pose["pose"]["heading"].as_f32().unwrap() }, velocity: pose["velocity"].as_f32().unwrap() });
        }
        sequence_count += 1;
    }
    if !json["leeway"].is_empty() {
        result.leeway = Some(HeadingWithVelocity { heading: json["leeway"]["heading"].as_f32().unwrap(), velocity: json["leeway"]["velocity"].as_f32().unwrap() });
    }
    if json["obstacles"].len() > 0 {
        for obstacle in json["obstacles"].members() {
            result.obstacles.push(Vec::new());
            for point in obstacle.members() {
                result.obstacles[obstacle_count].push(Point { x: point["x"].as_f32().unwrap(), y: point["y"].as_f32().unwrap() });
            }
            obstacle_count += 1;
        }
    }
    result
}

/// Visualization of multiple timed sequences
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Data file in CSV format
    #[arg(short, long, default_value_t = String::from(""))]
    csv_file: String,

    #[arg(short, long, default_value_t = String::from(""))]
    json_file: String,
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();
    if args.csv_file.len() == 0 && args.json_file.len() == 0 {
        return Err("No input file!");
    }
    let mut scene = Scene {
        motion_sequences: Vec::new(),
        obstacles: Vec::new(),
        leeway: None
    };
    if args.csv_file.len() > 0 {
        println!("Parsing CSV file {}...", args.csv_file);
        scene = get_scene_csv(args.csv_file);
    } else if args.json_file.len() > 0 {
        println!("Parsing JSON file {}...", args.json_file);
        scene = get_scene_json(args.json_file);
    }
    println!("Plotting scene...");
    let result = plot_scene(scene);
    println!("Done!");
    result
}
