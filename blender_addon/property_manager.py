import bpy

class ObjectSimulationSettings(bpy.types.PropertyGroup):
    is_graphics_object: bpy.props.BoolProperty(name="Enable Graphics", default=True)
    is_physics_object: bpy.props.BoolProperty(name="Enable Physics")
    mass: bpy.props.FloatProperty(name="Mass", min=0.00001, soft_min=0.00001, default=1, precision=4)
    body_type: bpy.props.EnumProperty(name="Body Type", items=[
        ("STATIC", "Static", "The object has infinite mass and is not affected by forces.", "", 1),
        ("KINEMATIC", "Kinematic", "The object is not affected by forces. It's position will be set manually.", "", 2),
        ("DYNAMIC", "Dynamic", "The object is fully physically simulated.", "", 3)
    ])
    optimized_shape: bpy.props.EnumProperty(name="Optimized Shape", default="NONE", items=[
        ("NONE", "None", "Have the simulation build a triangle mesh of this object. Note: This uses more memory than optimized shapes.", "", 1),
        ("CUBOID", "Cuboid", "Have the simulation use its intrinsic cuboid representation for this object.", "", 2),
        ("SPHERE", "Sphere", "Have the simulation use its intrinsic sphere representation for this object.", "", 3)
    ])
    is_point_of_interest: bpy.props.BoolProperty(name="Mark as Point of Interest", description="Store this object's location and rotation in the Perigee glTF using the object's name for use in-engine.")
    is_anonymous: bpy.props.BoolProperty(name="Make Anonymous", description="Don't make this object referenceable by name in the Perigee engine. This saves memory.", default=True)

    def to_dict(self):
        return {
            "graphics": {
                "enabled": self.is_graphics_object,
            },
            "physics": {
                "enabled": self.is_physics_object,
                "bodyType": self.body_type,
                "mass": self.mass,
                "optimizedShape": self.optimized_shape,
                "isAnonymous": self.is_anonymous
            },
            "isPointOfInterest": self.is_point_of_interest,
        }
    
    def from_dict(self, new_dict):
        self.is_graphics_object = new_dict["graphics"]["enabled"]
        self.is_physics_object = new_dict["physics"]["enabled"]
        self.body_type = new_dict["physics"]["bodyType"]
        self.mass = new_dict["physics"]["mass"]
        self.optimized_shape = new_dict["physics"]["optimizedShape"]
        self.is_anonymous = new_dict["physics"]["isAnonymous"]
        self.is_point_of_interest = new_dict["isPointOfInterest"]
        
class VIEW3D_PT_object_sim_settings(bpy.types.Panel):
    bl_space_type = "VIEW_3D"
    bl_region_type = "UI"
    # Tab name
    bl_category = "Perigee"
    # Dropdown name
    bl_label = "Sim Settings"

    @classmethod
    def poll(cls, context):
        return (context.object is not None)
    
    def draw_graphics_widgets(self, context_object, layout):
        layout.label(text = "Graphics")
        layout.prop(context_object.sim_settings, "is_graphics_object")

    def draw_physics_widgets(self, context_object, layout):
        layout.label(text = "Physics")
        layout.prop(context_object.sim_settings, "is_physics_object")
        if context_object.sim_settings.is_physics_object:
            if context_object.type == "MESH":
                if context_object.sim_settings.body_type != "DYNAMIC":
                    layout.prop(context_object.sim_settings, "is_anonymous")
                layout.prop(context_object.sim_settings, "body_type")
                layout.prop(context_object.sim_settings, "optimized_shape")
                if context_object.sim_settings.body_type == "DYNAMIC":
                    layout.prop(context_object.sim_settings, "mass")
            elif context_object.type == "EMPTY":
                layout.label(text = "Body Type: SENSOR")
            else:
                pass

    def draw(self, context):
        layout = self.layout
        if context.object:
            layout.prop(context.object, "name")
            layout.separator()
            layout.prop(context.object.sim_settings, "is_point_of_interest")
            graphics_col = layout.column(align = True)
            self.draw_graphics_widgets(context.object, graphics_col)
            layout.separator()
            physics_col = layout.column(align = True)
            self.draw_physics_widgets(context.object, physics_col)
        else:
            self.layout.label(text = "No object selected")

blender_classes = [
    ObjectSimulationSettings,
    VIEW3D_PT_object_sim_settings,
]

def register():
    for blender_class in blender_classes:
        bpy.utils.register_class(blender_class)
    bpy.types.Object.sim_settings = bpy.props.PointerProperty(name="Sim Settings", type=ObjectSimulationSettings)

def unregister():
    del bpy.types.Object.sim_settings
    for blender_class in blender_classes:
        bpy.utils.unregister_class(blender_class)