<template>
  <div id="app" class="p-4">
    <header class="flex items-center justify-between mb-4">
      <h1 class="text-2xl font-bold">comics-list</h1>
      <RouterLink class="btn btn-sm" to="/series">Series</RouterLink>
    </header>

    <RouterView />
  </div>
</template>

