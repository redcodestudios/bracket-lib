#include <lua.h>
#include <lauxlib.h>
#include <lualib.h> 

#include <stdio.h>

void call_lua_bytes(lua_State* state, const unsigned char* source, size_t size) {
    if(luaL_loadbuffer(state, source, size, "script xaaab")){
        fprintf(stderr, "Lua `ERROR`: `%s\n`", lua_tostring(state, -1));
    }
    if(lua_pcall(state, 0, 0, 0)){
        fprintf(stderr, "Lua `ERROR`: `%s\n`", lua_tostring(state, -1));
    }
}

void call_lua(const char* script) {
    lua_State *L;
    L = luaL_newstate();
    printf("C: loading lua script %s\n", script);
    luaL_openlibs(L);

    luaL_loadfile(L, script);

    if (lua_pcall(L, 0, 0, 0))
        printf("C: falhou: %s\n", lua_tostring(L, -1));
    
    lua_close(L);
}

char* call_lua_return(const char* script) {
    lua_State *L;
    L = luaL_newstate();
    printf("C: loading lua script %s\n", script);
    luaL_openlibs(L);

    luaL_loadfile(L, script);

    if (lua_pcall(L, 0, 0, 0))
	printf("C: falhou: %s\n", lua_tostring(L, -1));
    
    lua_getglobal(L, "text");
    fprintf(stderr, "aaaaaaaaaaa\n");
    char* str_from_script = lua_tostring(L, -1);
    fprintf(stderr, str_from_script);
    lua_close(L);
    return str_from_script;
}

void call_setup_lua(lua_State* state, const unsigned char* source, size_t size, const char* script) {
   
    if (luaL_loadbuffer(state, source, size, script) || lua_pcall(state, 0, 0, 0)) {
        printf("error buffer: %s", lua_tostring(state, -1));
        return -1;
    }
    int x = lua_getglobal(state, "setup");

    if(lua_pcall(state, 0, 1, 0) != 0) {
        printf("error running function `setup`: %s\n", lua_tostring(state, -1));
    }
}

void call_update_lua(lua_State* state, const unsigned char* source, size_t size, const char* script) {
   
    if (luaL_loadbuffer(state, source, size, script) || lua_pcall(state, 0, 0, 0)) {
        printf("error buffer: %s", lua_tostring(state, -1));
        return -1;
    }
    int x = lua_getglobal(state, "update");

    if(lua_pcall(state, 0, 1, 0) != 0) {
        printf("error running function `update`: %s\n", lua_tostring(state, -1));
    }
}