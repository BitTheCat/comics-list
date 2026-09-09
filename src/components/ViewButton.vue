<template>
    <button :title="t('button_view')">
        <Eye :size="16" />
    </button>
</template>

<script setup lang="ts">
import { Eye } from '@lucide/vue';
import { useI18n } from '../i18n/useI18n';

const { t } = useI18n();
</script>
