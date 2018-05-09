use dust::program;
use gl;
use dust::input;
use dust::material;
use gust::mesh;
use std::rc::Rc;

pub struct ColorMaterial {
    program: program::Program
}

impl material::Material for ColorMaterial
{
    fn apply(&self)
    {
        self.program.set_used();
    }

    fn setup_states(&self, _gl: &gl::Gl) -> Result<(), material::Error> {
        Ok(())
    }

    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), material::Error>
    {
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        Ok(())
    }

    fn setup_attributes(&self, mesh: &mesh::Mesh) -> Result<(), material::Error>
    {
        let mut list = Vec::new();
        list.push( mesh.positions());
        list.push(mesh.get("Color")?);
        self.program.add_attributes(&list)?;
        Ok(())
    }
}

impl ColorMaterial
{
    pub fn create(gl: &gl::Gl) -> Result<Rc<material::Material>, material::Error>
    {
        let shader_program = program::Program::from_resource(&gl, "examples/assets/shaders/color")?;

        Ok(Rc::new(ColorMaterial { program: shader_program }))
    }
}