//! Scatter trace

#[cfg(feature = "plotly_ndarray")]
use ndarray::{Array, Ix1, Ix2};
use serde::Serialize;

#[cfg(feature = "plotly_ndarray")]
use crate::ndarray::ArrayTraces;
use crate::{
    color::Color,
    common::{
        Calendar, Dim, ErrorData, Fill, Font, HoverInfo, HoverOn, Label, Line, Marker, Mode,
        Orientation, PlotType, Position, Visible,
    },
    private, Trace,
};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GroupNorm {
    #[serde(rename = "")]
    Default,
    Fraction,
    Percent,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum StackGaps {
    #[serde(rename = "infer zero")]
    InferZero,
    Interpolate,
}

/// Construct a scatter trace.
///
/// # Examples
///
/// ```
/// use plotly::Scatter;
///
/// let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
///
/// let expected = serde_json::json!({
///     "type": "scatter",
///     "x": [0, 1, 2],
///     "y": [2, 1, 0]
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct Scatter<X, Y>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    r#type: PlotType,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    opacity: Option<f64>,
    mode: Option<Mode>,
    ids: Option<Vec<String>>,
    x: Option<Vec<X>>,

    x0: Option<private::NumOrString>,
    dx: Option<f64>,

    y: Option<Vec<Y>>,

    y0: Option<private::NumOrString>,
    dy: Option<f64>,

    text: Option<Dim<String>>,
    #[serde(rename = "textposition")]
    text_position: Option<Dim<Position>>,
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,

    meta: Option<private::NumOrString>,
    #[serde(rename = "customdata")]
    custom_data: Option<private::NumOrStringCollection>,

    #[serde(rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(rename = "yaxis")]
    y_axis: Option<String>,
    orientation: Option<Orientation>,
    #[serde(rename = "groupnorm")]
    group_norm: Option<GroupNorm>,
    #[serde(rename = "stackgroup")]
    stack_group: Option<String>,
    marker: Option<Marker>,
    line: Option<Line>,
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
    error_x: Option<ErrorData>,
    error_y: Option<ErrorData>,
    #[serde(rename = "cliponaxis")]
    clip_on_axis: Option<bool>,
    #[serde(rename = "connectgaps")]
    connect_gaps: Option<bool>,
    fill: Option<Fill>,
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hoveron")]
    hover_on: Option<HoverOn>,
    #[serde(rename = "stackgaps")]
    stack_gaps: Option<StackGaps>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<X, Y> Default for Scatter<X, Y>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    fn default() -> Self {
        Scatter {
            r#type: PlotType::Scatter,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            mode: None,
            ids: None,
            x: None,
            x0: None,
            dx: None,
            y: None,
            y0: None,
            dy: None,
            text: None,
            text_position: None,
            text_template: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            meta: None,
            custom_data: None,
            x_axis: None,
            y_axis: None,
            orientation: None,
            group_norm: None,
            stack_group: None,
            marker: None,
            line: None,
            text_font: None,
            error_x: None,
            error_y: None,
            clip_on_axis: None,
            connect_gaps: None,
            fill: None,
            fill_color: None,
            hover_label: None,
            hover_on: None,
            stack_gaps: None,
            x_calendar: None,
            y_calendar: None,
        }
    }
}

impl<X, Y> Scatter<X, Y>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Box<Self> {
        Box::new(Self {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }

    #[cfg(feature = "plotly_ndarray")]
    pub fn from_array(x: Array<X, Ix1>, y: Array<Y, Ix1>) -> Box<Self> {
        Box::new(Scatter {
            x: Some(x.to_vec()),
            y: Some(y.to_vec()),
            ..Default::default()
        })
    }

    /// Produces `Scatter` traces from a 2 dimensional tensor (`traces_matrix`) indexed by `x`. This
    /// function requires the `ndarray` feature.
    ///
    /// # Arguments
    /// * `x`             - One dimensional array (or view) that represents the `x` axis coordinates.
    /// * `traces_matrix` - Two dimensional array (or view) containing the `y` axis coordinates of
    /// the traces.
    /// * `array_traces`  - Determines whether the traces are arranged in the matrix over the
    /// columns (`ArrayTraces::OverColumns`) or over the rows (`ArrayTraces::OverRows`).
    ///
    /// # Examples
    ///
    /// ```
    /// use plotly::common::Mode;
    /// use plotly::{Plot, Scatter, ArrayTraces};
    /// use ndarray::{Array, Ix1, Ix2};
    ///
    /// fn ndarray_to_traces() {
    ///     let n: usize = 11;
    ///     let t: Array<f64, Ix1> = Array::range(0., 10., 10. / n as f64);
    ///     let mut ys: Array<f64, Ix2> = Array::zeros((11, 11));
    ///     let mut count = 0.;
    ///     for mut row in ys.columns_mut() {
    ///         for index in 0..row.len() {
    ///             row[index] = count + (index as f64).powf(2.);
    ///         }
    ///         count += 1.;
    ///     }
    ///
    ///     let traces = Scatter::default()
    ///         .mode(Mode::LinesMarkers)
    ///         .to_traces(t, ys, ArrayTraces::OverColumns);
    ///
    ///     let mut plot = Plot::new();
    ///     plot.add_traces(traces);
    ///     plot.show();
    /// }
    /// fn main() -> std::io::Result<()> {
    ///     ndarray_to_traces();
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "plotly_ndarray")]
    pub fn to_traces(
        &self,
        x: Array<X, Ix1>,
        traces_matrix: Array<Y, Ix2>,
        array_traces: ArrayTraces,
    ) -> Vec<Box<dyn Trace>> {
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let mut trace_vectors = private::trace_vectors_from(traces_matrix, array_traces);
        trace_vectors.reverse();
        while !trace_vectors.is_empty() {
            let mut sc = Box::new(self.clone());
            sc.x = Some(x.to_vec());
            let data = trace_vectors.pop();
            if let Some(d) = data {
                sc.y = Some(d);
                traces.push(sc);
            }
        }

        traces
    }

    /// Enables WebGL.
    pub fn web_gl_mode(mut self, on: bool) -> Box<Self> {
        self.r#type = if on {
            PlotType::ScatterGL
        } else {
            PlotType::Scatter
        };
        Box::new(self)
    }

    /// Sets the trace name. The trace name appear as the legend item and on hover.
    pub fn name(mut self, name: &str) -> Box<Self> {
        self.name = Some(name.to_string());
        Box::new(self)
    }

    /// Determines whether or not this trace is visible. If `Visible::LegendOnly`, the trace is not
    /// drawn, but can appear as a legend item (provided that the legend itself is visible).
    pub fn visible(mut self, visible: Visible) -> Box<Self> {
        self.visible = Some(visible);
        Box::new(self)
    }

    /// Determines whether or not an item corresponding to this trace is shown in the legend.
    pub fn show_legend(mut self, show_legend: bool) -> Box<Self> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    /// Sets the legend group for this trace. Traces part of the same legend group hide/show at the
    /// same time when toggling legend items.
    pub fn legend_group(mut self, legend_group: &str) -> Box<Self> {
        self.legend_group = Some(legend_group.to_string());
        Box::new(self)
    }

    /// Sets the opacity of the trace.
    pub fn opacity(mut self, opacity: f64) -> Box<Self> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    /// Determines the drawing mode for this scatter trace. If the provided `Mode` includes
    /// "Text" then the `text` elements appear at the coordinates. Otherwise, the `text` elements
    /// appear on hover. If there are less than 20 points and the trace is not stacked then the
    /// default is `Mode::LinesMarkers`, otherwise it is `Mode::Lines`.
    pub fn mode(mut self, mode: Mode) -> Box<Self> {
        self.mode = Some(mode);
        Box::new(self)
    }

    /// Assigns id labels to each datum. These ids for object constancy of data points during
    /// animation. Should be an array of strings, not numbers or any other type.
    pub fn ids<S: AsRef<str>>(mut self, ids: Vec<S>) -> Box<Self> {
        let ids = private::owned_string_vector(ids);
        self.ids = Some(ids);
        Box::new(self)
    }

    /// Alternate to `x`. Builds a linear space of x coordinates. Use with `dx` where `x0` is the
    /// starting coordinate and `dx` the step.
    pub fn x0<V: Into<private::NumOrString>>(mut self, x0: V) -> Box<Self> {
        self.x0 = Some(x0.into());
        Box::new(self)
    }

    /// Sets the x coordinate step. See `x0` for more info.
    pub fn dx(mut self, dx: f64) -> Box<Self> {
        self.dx = Some(dx);
        Box::new(self)
    }

    /// Alternate to `y`. Builds a linear space of y coordinates. Use with `dy` where `y0` is the
    /// starting coordinate and `dy` the step.
    pub fn y0<V: Into<private::NumOrString>>(mut self, y0: V) -> Box<Self> {
        self.y0 = Some(y0.into());
        Box::new(self)
    }

    /// Sets the y coordinate step. See `y0` for more info.
    pub fn dy(mut self, dy: f64) -> Box<Self> {
        self.dy = Some(dy);
        Box::new(self)
    }

    /// Sets text elements associated with each (x,y) pair. If a single string, the same string
    /// appears over all the data points. If an array of string, the items are mapped in order to
    /// the this trace's (x,y) coordinates. If the trace `HoverInfo` contains a "text" flag and
    /// `hover_text` is not set, these elements will be seen in the hover labels.
    pub fn text(mut self, text: &str) -> Box<Self> {
        self.text = Some(Dim::Scalar(text.to_string()));
        Box::new(self)
    }

    /// Sets text elements associated with each (x,y) pair. If a single string, the same string
    /// appears over all the data points. If an array of string, the items are mapped in order to
    /// the this trace's (x,y) coordinates. If trace `HoverInfo` contains a "text" flag and
    /// `hover_text` is not set, these elements will be seen in the hover labels.
    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Self> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    /// Sets the positions of the `text` elements with respects to the (x,y) coordinates.
    pub fn text_position(mut self, text_position: Position) -> Box<Self> {
        self.text_position = Some(Dim::Scalar(text_position));
        Box::new(self)
    }

    /// Sets the positions of the `text` elements with respects to the (x,y) coordinates.
    pub fn text_position_array(mut self, text_position: Vec<Position>) -> Box<Self> {
        self.text_position = Some(Dim::Vector(text_position));
        Box::new(self)
    }

    /// Template string used for rendering the information text that appear on points. Note that
    /// this will override `textinfo`. Variables are inserted using %{variable}, for example
    /// "y: %{y}". Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}". See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3)
    /// for details on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format) for details
    /// on the date formatting syntax. Every attributes that can be specified per-point (the ones
    /// that are `arrayOk: true`) are available.
    pub fn text_template(mut self, text_template: &str) -> Box<Self> {
        self.text_template = Some(Dim::Scalar(text_template.to_string()));
        Box::new(self)
    }

    /// Template string used for rendering the information text that appear on points. Note that
    /// this will override `textinfo`. Variables are inserted using %{variable}, for example
    /// "y: %{y}". Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}". See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3)
    /// for details on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format) for details
    /// on the date formatting syntax. Every attributes that can be specified per-point (the ones
    /// that are `arrayOk: true`) are available.
    pub fn text_template_array<S: AsRef<str>>(mut self, text_template: Vec<S>) -> Box<Self> {
        let text_template = private::owned_string_vector(text_template);
        self.text_template = Some(Dim::Vector(text_template));
        Box::new(self)
    }

    /// Sets hover text elements associated with each (x,y) pair. If a single string, the same
    /// string appears over all the data points. If an array of string, the items are mapped in
    /// order to the this trace's (x,y) coordinates. To be seen, trace `HoverInfo` must contain a
    /// "Text" flag.
    pub fn hover_text(mut self, hover_text: &str) -> Box<Self> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_string()));
        Box::new(self)
    }

    /// Sets hover text elements associated with each (x,y) pair. If a single string, the same
    /// string appears over all the data points. If an array of string, the items are mapped in
    /// order to the this trace's (x,y) coordinates. To be seen, trace `HoverInfo` must contain a
    /// "Text" flag.
    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Self> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    /// Determines which trace information appear on hover. If `HoverInfo::None` or `HoverInfo::Skip`
    /// are set, no information is displayed upon hovering. But, if `HoverInfo::None` is set, click
    /// and hover events are still fired.
    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Self> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    /// Template string used for rendering the information that appear on hover box. Note that this
    /// will override `HoverInfo`. Variables are inserted using %{variable}, for example "y: %{y}".
    /// Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3_format for details
    /// on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format for details
    /// on the date formatting syntax. The variables available in `hovertemplate` are the ones
    /// emitted as event data described at this link https://plotly.com/javascript/plotlyjs-events/#event-data.
    /// Additionally, every attributes that can be specified per-point (the ones that are
    /// `arrayOk: true`) are available. Anything contained in tag `<extra>` is displayed in the
    /// secondary box, for example "<extra>{fullData.name}</extra>". To hide the secondary box
    /// completely, use an empty tag `<extra></extra>`.
    pub fn hover_template(mut self, hover_template: &str) -> Box<Self> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_string()));
        Box::new(self)
    }

    /// Template string used for rendering the information that appear on hover box. Note that this
    /// will override `HoverInfo`. Variables are inserted using %{variable}, for example "y: %{y}".
    /// Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3_format for details
    /// on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format for details
    /// on the date formatting syntax. The variables available in `hovertemplate` are the ones
    /// emitted as event data described at this link https://plotly.com/javascript/plotlyjs-events/#event-data.
    /// Additionally, every attributes that can be specified per-point (the ones that are
    /// `arrayOk: true`) are available. Anything contained in tag `<extra>` is displayed in the
    /// secondary box, for example "<extra>{fullData.name}</extra>". To hide the secondary box
    /// completely, use an empty tag `<extra></extra>`.
    pub fn hover_template_array<S: AsRef<str>>(mut self, hover_template: Vec<S>) -> Box<Self> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    /// Assigns extra meta information associated with this trace that can be used in various text
    /// attributes. Attributes such as trace `name`, graph, axis and colorbar `title.text`,
    /// annotation `text` `rangeselector`, `updatemenues` and `sliders` `label` text all support
    /// `meta`. To access the trace `meta` values in an attribute in the same trace, simply use
    /// `%{meta[i]}` where `i` is the index or key of the `meta` item in question. To access trace
    /// `meta` in layout attributes, use `%{data[n[.meta[i]}` where `i` is the index or key of the
    /// `meta` and `n` is the trace index.
    pub fn meta<V: Into<private::NumOrString>>(mut self, meta: V) -> Box<Self> {
        self.meta = Some(meta.into());
        Box::new(self)
    }

    /// Assigns extra data each datum. This may be useful when listening to hover, click and
    /// selection events. Note that, "scatter" traces also appends customdata items in the markers
    /// DOM elements
    pub fn custom_data<V: Into<private::NumOrString> + Clone>(
        mut self,
        custom_data: Vec<V>,
    ) -> Box<Self> {
        self.custom_data = Some(custom_data.into());
        Box::new(self)
    }

    /// Sets a reference between this trace's x coordinates and a 2D cartesian x axis. If "x" (
    /// the default value), the x coordinates refer to `Layout::x_axis`. If "x2", the x coordinates
    /// refer to `Layout::x_axis2`, and so on.
    pub fn x_axis(mut self, axis: &str) -> Box<Self> {
        self.x_axis = Some(axis.to_string());
        Box::new(self)
    }

    /// Sets a reference between this trace's y coordinates and a 2D cartesian y axis. If "y"
    /// (the default value), the y coordinates refer to `Layout::y_axis`. If "y2", the y coordinates
    /// refer to `Layout::y_axis2`, and so on.
    pub fn y_axis(mut self, axis: &str) -> Box<Self> {
        self.y_axis = Some(axis.to_string());
        Box::new(self)
    }

    /// Only relevant when `stackgroup` is used, and only the first `orientation` found in the
    /// `stackgroup` will be used - including if `visible` is "legendonly" but not if it is `false`.
    /// Sets the stacking direction. With "v" ("h"), the y (x) values of subsequent traces are
    /// added. Also affects the default value of `fill`.
    pub fn orientation(mut self, orientation: Orientation) -> Box<Self> {
        self.orientation = Some(orientation);
        Box::new(self)
    }

    /// Only relevant when `stackgroup` is used, and only the first `groupnorm` found in the
    /// `stackgroup` will be used - including if `visible` is "legendonly" but not if it is `false`.
    /// Sets the normalization for the sum of this `stackgroup`. With "fraction", the value of each
    /// trace at each location is divided by the sum of all trace values at that location. "percent"
    /// is the same but multiplied by 100 to show percentages. If there are multiple subplots, or
    /// multiple `stackgroup`s on one subplot, each will be normalized within its own set.
    pub fn group_norm(mut self, group_norm: GroupNorm) -> Box<Self> {
        self.group_norm = Some(group_norm);
        Box::new(self)
    }

    /// Set several scatter traces (on the same subplot) to the same stackgroup in order to add
    /// their y values (or their x values if `orientation` is "h"). If blank or omitted this trace
    /// will not be stacked. Stacking also turns `fill` on by default, using "tonexty" ("tonextx")
    /// if `orientation` is "h" ("v") and sets the default `mode` to "lines" irrespective of point
    /// count. You can only stack on a numeric (linear or log) axis. Traces in a `stackgroup` will
    /// only fill to (or be filled to) other traces in the same group. With multiple `stackgroup`s
    /// or some traces stacked and some not, if fill-linked traces are not already consecutive, the
    /// later ones will be pushed down in the drawing order.
    pub fn stack_group(mut self, stack_group: &str) -> Box<Self> {
        self.stack_group = Some(stack_group.to_string());
        Box::new(self)
    }

    /// Determines how points are displayed and joined.
    pub fn marker(mut self, marker: Marker) -> Box<Self> {
        self.marker = Some(marker);
        Box::new(self)
    }

    /// Line display properties.
    pub fn line(mut self, line: Line) -> Box<Self> {
        self.line = Some(line);
        Box::new(self)
    }

    /// Sets the text font.
    pub fn text_font(mut self, text_font: Font) -> Box<Self> {
        self.text_font = Some(text_font);
        Box::new(self)
    }

    /// x-axis error display properties.
    pub fn error_x(mut self, error_x: ErrorData) -> Box<Self> {
        self.error_x = Some(error_x);
        Box::new(self)
    }

    /// y-axis error display properties.
    pub fn error_y(mut self, error_y: ErrorData) -> Box<Self> {
        self.error_y = Some(error_y);
        Box::new(self)
    }

    /// Determines whether or not markers and text nodes are clipped about the subplot axes. To show
    /// markers and text nodes above axis lines and tick labels, make sure to set `xaxis.layer` and
    /// `yaxis.layer` to "below traces".
    pub fn clip_on_axis(mut self, clip_on_axis: bool) -> Box<Self> {
        self.clip_on_axis = Some(clip_on_axis);
        Box::new(self)
    }

    /// Determines whether or not gaps (i.e. {nan} or missing values) in the provided data arrays
    /// are connected.
    pub fn connect_gaps(mut self, connect_gaps: bool) -> Box<Self> {
        self.connect_gaps = Some(connect_gaps);
        Box::new(self)
    }

    /// Sets the area to fill with a solid color. Defaults to "none" unless this trace is stacked,
    /// then it gets "tonexty" ("tonextx") if `orientation` is "v" ("h") Use with `fillcolor` if not
    /// "none". "tozerox" and "tozeroy" fill to x=0 and y=0 respectively. "tonextx" and "tonexty"
    /// fill between the endpoints of this trace and the endpoints of the trace before it,
    /// connecting those endpoints with straight lines (to make a stacked area graph); if there is
    /// no trace before it, they behave like "tozerox" and "tozeroy". "toself" connects the
    /// endpoints of the trace (or each segment of the trace if it has gaps) into a closed shape.
    /// "tonext" fills the space between two traces if one completely encloses the other
    /// (eg consecutive contour lines), and behaves like "toself" if there is no trace before it.
    /// "tonext" should not be used if one trace does not enclose the other. Traces in a
    /// `stackgroup` will only fill to (or be filled to) other traces in the same group. With
    /// multiple `stackgroup`s or some traces stacked and some not, if fill-linked traces are not
    /// already consecutive, the later ones will be pushed down in the drawing order.
    pub fn fill(mut self, fill: Fill) -> Box<Self> {
        self.fill = Some(fill);
        Box::new(self)
    }

    /// Sets the fill color. Defaults to a half-transparent variant of the line color, marker color,
    /// or marker line color, whichever is available.
    pub fn fill_color<C: Color>(mut self, fill_color: C) -> Box<Self> {
        self.fill_color = Some(Box::new(fill_color));
        Box::new(self)
    }

    /// Properties of label displayed on mouse hover.
    pub fn hover_label(mut self, hover_label: Label) -> Box<Self> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    /// Do the hover effects highlight individual points (markers or line points) or do they
    /// highlight filled regions? If the fill is "toself" or "tonext" and there are no markers or
    /// text, then the default is "fills", otherwise it is "points".
    pub fn hover_on(mut self, hover_on: HoverOn) -> Box<Self> {
        self.hover_on = Some(hover_on);
        Box::new(self)
    }

    /// Only relevant when `stack_group` is used, and only the first `stack_gaps` found in the
    /// `stackgroup` will be used - including if `visible` is set to `Visible::LegendOnly` but not
    /// if it is set to `Visible::False`.
    /// Determines how we handle locations at which other traces in this group have data but this
    /// one does not. With "infer zero" we insert a zero at these locations. With "interpolate" we
    /// linearly interpolate between existing values, and extrapolate a constant beyond the existing
    /// values.
    pub fn stack_gaps(mut self, stack_gaps: StackGaps) -> Box<Self> {
        self.stack_gaps = Some(stack_gaps);
        Box::new(self)
    }

    /// Sets the calendar system to use with `x` date data.
    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Self> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    /// Sets the calendar system to use with `y` date data.
    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Self> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }
}

impl<X, Y> Trace for Scatter<X, Y>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_serialize_group_norm() {
        assert_eq!(to_value(GroupNorm::Default).unwrap(), json!(""));
        assert_eq!(to_value(GroupNorm::Fraction).unwrap(), json!("fraction"));
        assert_eq!(to_value(GroupNorm::Percent).unwrap(), json!("percent"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_stack_gaps() {
        assert_eq!(to_value(StackGaps::InferZero).unwrap(), json!("infer zero"));
        assert_eq!(to_value(StackGaps::Interpolate).unwrap(), json!("interpolate"));
    }

    #[test]
    fn test_serialize_default_scatter() {
        let trace = Scatter::<u32, u32>::default();
        let expected = json!({"type": "scatter"});

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_scatter() {
        use crate::common::ErrorType;

        let trace = Scatter::new(vec![0, 1], vec![2, 3])
            .clip_on_axis(true)
            .connect_gaps(false)
            .custom_data(vec!["custom_data"])
            .error_x(ErrorData::new(ErrorType::Percent))
            .error_y(ErrorData::new(ErrorType::Data))
            .dx(1.0)
            .dy(4.0)
            .fill(Fill::ToNext)
            .fill_color("#789456")
            .group_norm(GroupNorm::Default)
            .hover_info(HoverInfo::Name)
            .hover_label(Label::new())
            .hover_on(HoverOn::Points)
            .hover_text("hover_text")
            .hover_text_array(vec!["hover_text"])
            .hover_template("hover_template")
            .hover_template_array(vec!["hover_template"])
            .ids(vec!["1"])
            .legend_group("legend_group")
            .line(Line::new())
            .marker(Marker::new())
            .meta("meta")
            .mode(Mode::LinesMarkers)
            .name("scatter_trace")
            .opacity(0.6)
            .orientation(Orientation::Horizontal)
            .show_legend(false)
            .stack_gaps(StackGaps::InferZero)
            .stack_group("stack_group")
            .text("text")
            .text_array(vec!["text"])
            .text_font(Font::new())
            .text_position(Position::MiddleCenter)
            .text_position_array(vec![Position::MiddleLeft])
            .text_template("text_template")
            .text_template_array(vec!["text_template"])
            .visible(Visible::True)
            .x_axis("x_axis")
            .x_calendar(Calendar::Chinese)
            .x0(0)
            .y_axis("y_axis")
            .y_calendar(Calendar::Coptic)
            .y0(2)
            .web_gl_mode(true);

        let expected = json!({
            "type": "scattergl",
            "x": [0, 1],
            "y": [2, 3],
            "cliponaxis": true,
            "connectgaps": false,
            "customdata": ["custom_data"],
            "error_x": {"type": "percent"},
            "error_y": {"type": "data"},
            "dx": 1.0,
            "dy": 4.0,
            "fill": "tonext",
            "fillcolor": "#789456",
            "groupnorm": "",
            "hoverinfo": "name",
            "hoverlabel": {},
            "hoveron": "points",
            "hovertext": ["hover_text"],
            "hovertemplate": ["hover_template"],
            "ids": ["1"],
            "legendgroup": "legend_group",
            "line": {},
            "marker": {},
            "meta": "meta",
            "mode": "lines+markers",
            "name": "scatter_trace",
            "opacity": 0.6,
            "orientation": "h",
            "showlegend": false,
            "stackgaps": "infer zero",
            "stackgroup": "stack_group",
            "text": ["text"],
            "textfont": {},
            "textposition": ["middle left"],
            "texttemplate": ["text_template"],
            "visible": true,
            "xaxis": "x_axis",
            "xcalendar": "chinese",
            "x0": 0,
            "yaxis": "y_axis",
            "ycalendar": "coptic",
            "y0": 2
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
