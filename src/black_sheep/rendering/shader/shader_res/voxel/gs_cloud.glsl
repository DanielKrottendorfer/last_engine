#version 450
#extension GL_EXT_gpu_shader4 : enable

layout (points) in;
layout (triangle_strip, max_vertices = 15) out;

in VS_OUT {
    float color;
} gs_in[];

out GS_OUT {
    float color;
} gs_out;

uniform isampler2D triTableTex;
uniform mat4 mv;
uniform mat4 projection;

int triTableValue(int i, int j){
    float fi = float(j) / 15.0;
    float fj = float(i) / 255.0;
    return texture(triTableTex, vec2(fi, fj)).r;
}

void build_house(vec4 position)
{    
    if (triTableValue(254,1) ==  3 ){
        gs_out.color = gs_in[0].color;
        gl_Position = projection * (position + vec4(-0.02,  -0.02, 0.0, 0.0));    // 3:bot-left
        EmitVertex();
        gs_out.color = gs_in[0].color;
        gl_Position = projection * (position + vec4( 0.02,  -0.02, 0.0, 0.0));    // 4:bot-right
        EmitVertex();
        gs_out.color = gs_in[0].color;
        gl_Position = projection * (position + vec4( 0.0,  0.02, 0.0, 0.0));    // 5:topZ
        EmitVertex();
        EndPrimitive();
    }else{
        gs_out.color = 0.5;
        gl_Position = projection * (position + vec4(-0.02,  -0.02, 0.0, 0.0));    // 3:bot-left
        EmitVertex();
        gs_out.color = 0.5;
        gl_Position = projection * (position + vec4( 0.02,  -0.02, 0.0, 0.0));    // 4:bot-right
        EmitVertex();
        gs_out.color = 0.5;
        gl_Position = projection * (position + vec4( 0.0,  0.02, 0.0, 0.0));    // 5:top
        EmitVertex();
        EndPrimitive();
    }

}

vec3 vertexInterp(float isolevel, vec3 v0, float l0, vec3 v1, float l1){
    return mix(v0, v1, (isolevel-l0)/(l1-l0));
}

vec3 cubePos(int i,vec3 position){

    float size = 0.01;

    int t = i;
    vec3 cp = vec3(0.0,0.0,0.0);

    if (t>3){
        cp.y = size;
        t = t - 4;
    }

    if (t==1){
        cp.x = size;
    }else if (t==2){
        cp.x = size;
        cp.z = size;
    }else if (t==3){
        cp.z = size;
    }

    return position + cp;
}

float cubeVal(int i){



    vec3 center = vec3(0.45,0.45,0.45);


    float R = 0.3;
    vec3 pos = cubePos(i,gl_in[0].gl_Position.xyz) - center;

    float r = sqrt(pow(R-sqrt(pow(pos.x,2.0)+pow(pos.z,2.0)),2.0) + pow(pos.y,2.0));


    if (r > 0.15 ){
        return 1.0;
    }else{
        return 0.0;
    }

    // float l = length(center-pos);
    // if(i==0){
    //     return 1.0;
    // }else
    // if(i==1){
    //     return 1.0;
    // }else{
    //     return 0.0;
    // }
}

int marching_cubes(vec4 position)
{
    float isolevel = 0.5;
    int cubeindex = 0;
    float cubeVal0 = cubeVal(0);
    float cubeVal1 = cubeVal(1);
    float cubeVal2 = cubeVal(2);
    float cubeVal3 = cubeVal(3);
    float cubeVal4 = cubeVal(4);
    float cubeVal5 = cubeVal(5);
    float cubeVal6 = cubeVal(6);
    float cubeVal7 = cubeVal(7);
    //Determine the index into the edge table which
    //tells us which vertices are inside of the surface
    cubeindex = int(cubeVal0 < isolevel);
    cubeindex += int(cubeVal1 < isolevel)*2;
    cubeindex += int(cubeVal2 < isolevel)*4;
    cubeindex += int(cubeVal3 < isolevel)*8;
    cubeindex += int(cubeVal4 < isolevel)*16;
    cubeindex += int(cubeVal5 < isolevel)*32;
    cubeindex += int(cubeVal6 < isolevel)*64;
    cubeindex += int(cubeVal7 < isolevel)*128;
    //Cube is entirely in/out of the surface
    if (cubeindex ==0 || cubeindex == 255)
        return 0;
    vec3 vertlist[12];
    //Find the vertices where the surface intersects the cube
    vertlist[0] = vertexInterp(isolevel, cubePos(0,position.xyz), cubeVal0, cubePos(1,position.xyz), cubeVal1);
    vertlist[1] = vertexInterp(isolevel, cubePos(1,position.xyz), cubeVal1, cubePos(2,position.xyz), cubeVal2);
    vertlist[2] = vertexInterp(isolevel, cubePos(2,position.xyz), cubeVal2, cubePos(3,position.xyz), cubeVal3);
    vertlist[3] = vertexInterp(isolevel, cubePos(3,position.xyz), cubeVal3, cubePos(0,position.xyz), cubeVal0);
    vertlist[4] = vertexInterp(isolevel, cubePos(4,position.xyz), cubeVal4, cubePos(5,position.xyz), cubeVal5);
    vertlist[5] = vertexInterp(isolevel, cubePos(5,position.xyz), cubeVal5, cubePos(6,position.xyz), cubeVal6);
    vertlist[6] = vertexInterp(isolevel, cubePos(6,position.xyz), cubeVal6, cubePos(7,position.xyz), cubeVal7);
    vertlist[7] = vertexInterp(isolevel, cubePos(7,position.xyz), cubeVal7, cubePos(4,position.xyz), cubeVal4);
    vertlist[8] = vertexInterp(isolevel, cubePos(0,position.xyz), cubeVal0, cubePos(4,position.xyz), cubeVal4);
    vertlist[9] = vertexInterp(isolevel, cubePos(1,position.xyz), cubeVal1, cubePos(5,position.xyz), cubeVal5);
    vertlist[10] = vertexInterp(isolevel, cubePos(2,position.xyz), cubeVal2, cubePos(6,position.xyz), cubeVal6);
    vertlist[11] = vertexInterp(isolevel, cubePos(3,position.xyz), cubeVal3, cubePos(7,position.xyz), cubeVal7);
    // Create the triangle
    //gl_FrontColor=vec4(cos(isolevel*5.0-0.5), sin(isolevel*5.0-0.5), 0.5, 1.0);
    int i=0;
    //Strange bug with this way, uncomment to test
    //for (i=0; triTableValue(cubeindex, i)!=-1; i+=3) {
    //int x = 0;

    while(true){
        if(triTableValue(cubeindex, i)!=-1){
            //Generate first vertex of triangle//
            //Fill position varying attribute for fragment shader
            vec4 p = vec4(vertlist[triTableValue(cubeindex, i)], 1);
            //Fill gl_Position attribute for vertex raster space position
            gs_out.color = gs_in[0].color;
            gl_Position = projection * mv * p;
            EmitVertex();
            //Generate second vertex of triangle//
            //Fill position varying attribute for fragment shader
            p = vec4(vertlist[triTableValue(cubeindex, i+1)], 1);
            gs_out.color = gs_in[0].color;
            //Fill gl_Position attribute for vertex raster space position
            gl_Position = projection * mv * p;
            EmitVertex();
            //Generate last vertex of triangle//
            //Fill position varying attribute for fragment shader
            p = vec4(vertlist[triTableValue(cubeindex, i+2)], 1);
            gs_out.color = gs_in[0].color;
            //Fill gl_Position attribute for vertex raster space position
            gl_Position = projection * mv * p;
            EmitVertex();
            //End triangle strip at firts triangle
            EndPrimitive();
        }else{
            break;
        }
        //x = x + 1;
        i=i+3; //Comment it for testing the strange bug
    }
    return 1;
}


void main()
{
    marching_cubes(gl_in[0].gl_Position);
    // vec3 v = marching_cubes(gl_in[0].gl_Position);
    // gs_out.color = v.x;
    //build_house(mv * gl_in[0].gl_Position);
}  

