<template>
  <div id="app" class="p-4">
    <header class="flex items-center justify-between mb-4">
      <h1 class="text-2xl font-bold">comics-list</h1>
      <div class="breadcrumbs text-sm">
        <RouterLink to="/series">Series</RouterLink>
      </div>
    </header>

    <RouterView />
  </div>
</template>

