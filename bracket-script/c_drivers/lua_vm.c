#include <lua.h>
#include <lauxlib.h>
#include <lualib.h> 

#include <stdio.h>

void test_call_lua(lua_State* state, const unsigned char* source, size_t size) {
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

void call_on_start_lua(const char* script) {
    lua_State* L;
    L = luaL_newstate();
    luaL_openlibs(L);

    luaL_dofile(L, script);
     
    lua_getglobal(L, "start");

    if(lua_pcall(L, 0, 0, 0) != 0) {
        printf("error running function `on_start`: %s\n", lua_tostring(L, -1));
    }

    lua_close(L);
}
