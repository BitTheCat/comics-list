<template>
    <button :title="t('button_edit')">
        <Pencil :size="16" />
    </button>
</template>

<script setup lang="ts">
import { Pencil } from '@lucide/vue';
import { useI18n } from '../i18n/useI18n';

const { t } = useI18n();
</script>
