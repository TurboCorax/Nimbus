pub struct Circuit {
    pub name: String,
    pub nodes: Vec<Node>,
    pub components: Vec<Component>,
    pub models: Vec<Model>,
    // pub subcircuits: Vec<Subcircuit>,
    pub sim_params: SimulationParams,
    pub external_registry: ExternalRegistry,
}

struct SimulationParams {
    sim_type: SimulationType, // 仿真类型（如 DC, TRAN, AC）
    time_step: Option<f64>,   // 时间步长（用于瞬态仿真）
    stop_time: Option<f64>,   // 仿真结束时间
    start_freq: Option<f64>,  // 起始频率（用于 AC 仿真）
    stop_freq: Option<f64>,   // 结束频率
}

enum SimulationType {
    DC,
    AC,
    TRAN,
}

struct ExternalRegistry {
    // external components
}

enum Node {
    Node(u32),
    Ground,
    Vdd,
}

// any specific components
enum Component {}

// common properties of a group of components
enum Model {
    Resistor(Resistor),
    Capacitor(Capacitor),
    Inductor(Inductor),
    VoltageSource(VoltageSource),
    CurrentSource(CurrentSource),
}

struct Resistor {
    resistance: f32,
}

struct Capacitor {
    capacitance: f32,
}

struct Inductor {
    inductance: f32,
}

struct VoltageSource {
    voltage: f32,
}

struct CurrentSource {
    current: f32,
}
