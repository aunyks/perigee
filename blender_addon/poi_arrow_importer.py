import os
import bpy
from bpy.types import Operator
from bpy.props import FloatVectorProperty
from bpy_extras.object_utils import AddObjectHelper, object_data_add
from mathutils import Vector

def add_poi_arrow(operator, ctx):
    arrow_blend_filepath = os.path.join(os.path.dirname(os.path.realpath(__file__)), "resources", "meshes", "arrow.blend")
    # https://docs.blender.org/api/current/bpy.types.BlendDataLibraries.html#bpy.types.BlendDataLibraries.load
    with bpy.data.libraries.load(arrow_blend_filepath, link=False, relative=False) as (data_from, data_to):
        data_to.objects = [name for name in data_from.objects if name.startswith('Arrow')]
    for obj in data_to.objects:
        if obj is not None:
            ctx.view_layer.active_layer_collection.collection.objects.link(obj)

class OBJECT_OT_add_poi_mesh(Operator, AddObjectHelper):
    """Create a new Mesh Object"""
    bl_idname = "mesh.add_poi"
    bl_label = "Add Point of Interest"
    bl_options = {'REGISTER', 'UNDO'}

    def execute(self, context):
        add_poi_arrow(self, context)
        return {'FINISHED'}


# Registration

def add_object_button(self, context):
    self.layout.operator(
        OBJECT_OT_add_poi_mesh.bl_idname,
        text="Add Point of Interest",
        icon='SORT_DESC'
    )


# This allows us to right click on a button and link to documentation
def add_object_manual_map():
    # url_manual_prefix = "https://docs.blender.org/manual/en/latest/"
    # url_manual_mapping = (
    #     ("bpy.ops.mesh.add_object", "scene_layout/object/types.html"),
    # )
    # return url_manual_prefix, url_manual_mapping
    return "", [("", ""),]

def register():
    bpy.utils.register_class(OBJECT_OT_add_poi_mesh)
    bpy.utils.register_manual_map(add_object_manual_map)
    bpy.types.VIEW3D_MT_mesh_add.append(add_object_button)

def unregister():
    bpy.utils.unregister_class(OBJECT_OT_add_poi_mesh)
    bpy.utils.unregister_manual_map(add_object_manual_map)
    bpy.types.VIEW3D_MT_mesh_add.remove(add_object_button)