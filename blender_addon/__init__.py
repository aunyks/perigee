bl_info = {
    "name": "Perigee Realtime Engine Addon",
    "category": "Generic",
    "version": (0, 2, 0),
    "blender": (2, 80, 0),
    "location": "3D Viewport Side Panel & File > Export > glTF 2.0",
    "description": "An addon for specifying simulation scenes for the Perigee realtime engine.",
    "tracker_url": "https://github.com/aunyks/perigee/issues/",
    "author": "Gerald Nash",
}

import bpy
from . import property_manager
from . import poi_arrow_importer

def register():
    bpy.utils.register_class(PerigeeGltfExportProperties)
    bpy.types.Scene.PerigeeGltfExportProperties = bpy.props.PointerProperty(type=PerigeeGltfExportProperties)
    property_manager.register()
    poi_arrow_importer.register()

def unregister():
    unregister_panel()
    del bpy.types.Scene.PerigeeGltfExportProperties
    bpy.utils.unregister_class(PerigeeGltfExportProperties)
    property_manager.unregister()
    poi_arrow_importer.unregister()

def register_panel():
    try:
        bpy.utils.register_class(GLTF_PT_UserExtensionExportPanel)
        bpy.utils.register_class(GLTF_PT_UserExtensionImportPanel)
    except Exception:
        pass
    return unregister_panel

def unregister_panel():
    try:
        bpy.utils.unregister_class(GLTF_PT_UserExtensionExportPanel)
        bpy.utils.unregister_class(GLTF_PT_UserExtensionImportPanel)
    except Exception:
        pass

class PerigeeGltfExportProperties(bpy.types.PropertyGroup):
    enabled: bpy.props.BoolProperty(name = "Include Perigee Engine Extras", description = "Activate the Perigee Engine glTF extras", default = True)

class GLTF_PT_UserExtensionExportPanel(bpy.types.Panel):
    bl_space_type = "FILE_BROWSER"
    bl_region_type = "TOOL_PROPS"
    bl_label = "Enabled"
    bl_parent_id = "GLTF_PT_export_user_extensions"
    bl_options = {"DEFAULT_CLOSED"}

    @classmethod
    def poll(cls, context):
        sfile = context.space_data
        operator = sfile.active_operator
        return operator.bl_idname == "EXPORT_SCENE_OT_gltf"

    def draw_header(self, context):
        props = bpy.context.scene.PerigeeGltfExportProperties
        self.layout.prop(props, "enabled")

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        layout.use_property_decorate = False  # No animation.

        props = bpy.context.scene.PerigeeGltfExportProperties
        layout.active = props.enabled

class glTF2ExportUserExtension:
    def __init__(self):
        self.properties = bpy.context.scene.PerigeeGltfExportProperties

    def gather_node_hook(self, gltf2_object, blender_object, export_settings):
        if self.properties.enabled:
            # We use extras instead of extensions because the Rust 
            # glTF loader doesn't support retrieving extensions
            if gltf2_object.extras is None:
                gltf2_object.extras = {}
            gltf2_object.extras = {
                "simSettings": blender_object.sim_settings.to_dict()
            }
            if blender_object.type != "MESH":
                gltf2_object.extras["simSettings"]["physics"]["isAnonymous"] = False
            if blender_object.type == "EMPTY":
                # Overwrite
                gltf2_object.extras["simSettings"]["physics"]["bodyType"] = "SENSOR"
                # Newly create
                gltf2_object.extras["simSettings"]["physics"]["baseScale"] = [blender_object.empty_display_size, blender_object.empty_display_size, blender_object.empty_display_size]

                optimized_shape = "CUBOID"
                if blender_object.empty_display_type == "SPHERE":
                    optimized_shape = "SPHERE"
                gltf2_object.extras["simSettings"]["physics"]["optimizedShape"] = optimized_shape
            elif blender_object.type == "MESH":
                gltf2_object.extras["simSettings"]["physics"]["baseScale"] = [blender_object.dimensions[0] / blender_object.scale[0], blender_object.dimensions[1] / blender_object.scale[1], blender_object.dimensions[2] / blender_object.scale[2]]
                if gltf2_object.extras["simSettings"]["physics"]["bodyType"] == "DYNAMIC":
                    gltf2_object.extras["simSettings"]["physics"]["isAnonymous"] = False
            else:
                pass
    
    def gather_scene_hook(self, gltf2_scene, blender_scene, export_settings):
        if self.properties.enabled:
            if gltf2_scene.extras is None:
                gltf2_scene.extras = {}
            gltf2_scene.extras = {
                "perigeeBlenderAddonVersion": [0, 2, 0]
            }

class glTF2ImportUserExtension:
    def __init__(self):
        pass

    def gather_import_node_after_hook(self, vnode, gltf2_node, blender_object, gltf):
        if gltf2_node.extras is not None:
            blender_object.sim_settings.from_dict(gltf2_node.extras["simSettings"])
