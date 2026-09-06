<template>
<div class="overflow-x-auto border rounded-box border-base-content/5 my-2">
  <table class="table table-zebra">
    <thead>
      <tr>
        <th v-for="value in fields" :key="value.key">{{ value.label }}</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="item in items" :key="item.id">
        <td v-for="value in fields" :key="value.key">
            <slot :name="value.key" :data="getValue(item, value.key)" :item="item">
                {{ getValue(item, value.key) }}
            </slot>
        </td>
      </tr>
    </tbody>
  </table>
</div>
</template>
<script lang="ts" setup>
import { PropType } from 'vue';

interface Field {
  key: string;
  label: string;
}

const props = defineProps({
  items: Array as PropType<any[]>,
  fields: Array as PropType<Field[]>,
})

function getValue(obj: any, path?: string) {
  if (obj == null || !path) return undefined;
  const parts = path.replace(/\[(\w+)\]/g, '.$1').split('.');
  return parts.reduce((o: any, key: string) => (o == null ? undefined : o[key]), obj);
}

</script>