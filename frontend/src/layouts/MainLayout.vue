<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar>
        <q-toolbar-title> Frank Ottey's SE577 SPA Application </q-toolbar-title>
        <div>v 0.1</div>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <div id="app">
        <div id="nav">
          <router-link to="/">Home</router-link> |
          <router-link to="/login">Login</router-link> |
          <router-link to="/about">Info About Me</router-link> |
          <router-link to="/repositories">Repositories</router-link> |
          <router-link to="/details">Repository Details</router-link>
        </div>
        <router-view />
      </div>
    </q-page-container>
  </q-layout>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
#nav {
  padding: 30px;
}
#nav a {
  font-weight: bold;
  color: #2c3e50;
}
#nav a.router-link-exact-active {
  color: #42b983;
}
</style>
