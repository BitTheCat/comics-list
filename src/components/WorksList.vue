<template>
  <div>
    <ul>
      <li v-for="w in works" :key="w.id">{{ w.title }}</li>
    </ul>
    <form @submit.prevent="create">
      <input v-model="title" placeholder="New work title" />
      <br />
      <textarea v-model="description" placeholder="Description (optional)"></textarea>
      <br />
      <button type="submit">Create</button>
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, PropType } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

export default defineComponent({
  props: {
    works: {
      type: Array as PropType<any[]>,
      default: () => [],
    },
  },
  emits: ['created'],
  setup(_, { emit }) {
    const title = ref('');
    const description = ref('');

    const create = async () => {
      if (!title.value) return;
      try {
        await invoke('api_create_work', {
          payload: { title: title.value, description: description.value || null },
        });
        title.value = '';
        description.value = '';
        emit('created');
      } catch (e) {
        console.error(e);
      }
    };

    return { title, description, create };
  },
});
</script>

