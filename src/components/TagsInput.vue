<template>
    <label class="form-group">
        <span v-if="label" class="label">{{ label }}</span>
        <div class="flex gap-2">
            <input
                :id="inputId"
                v-model="inputValue"
                class="input flex-1"
                :name="name"
                :placeholder="placeholder"
                @keydown="onKeydown"
                @blur="addTags"
            />
            <button class="btn" type="button" @click="addTags">
                {{ addLabel }}
            </button>
        </div>
        <div v-if="modelValue.length" class="mt-2 flex flex-wrap gap-2">
            <span
                v-for="(tag, index) in modelValue"
                :key="tag"
                class="badge badge-outline gap-1"
            >
                {{ tag }}
                <button
                    type="button"
                    :aria-label="`Remove tag ${tag}`"
                    class="cursor-pointer"
                    @click="removeTag(index)"
                >
                    ×
                </button>
            </span>
        </div>
    </label>
</template>

<script setup lang="ts">
import { ref } from "vue";

const props = withDefaults(
    defineProps<{
        modelValue: string[];
        inputId?: string;
        name?: string;
        label?: string;
        placeholder?: string;
        addLabel?: string;
    }>(),
    {
        inputId: undefined,
        name: "tags",
        label: "Tags",
        placeholder: "Add tags separated by commas",
        addLabel: "Add",
    },
);

const emit = defineEmits<{
    (e: "update:modelValue", value: string[]): void;
}>();

const inputValue = ref("");

const addTags = () => {
    const tags = [...props.modelValue];
    const newTags = inputValue.value
        .split(/[,;]/)
        .map((tag) => tag.trim())
        .filter(Boolean);

    for (const tag of newTags) {
        const alreadyExists = tags.some(
            (currentTag) =>
                currentTag.toLocaleLowerCase() === tag.toLocaleLowerCase(),
        );
        if (!alreadyExists) tags.push(tag);
    }

    if (newTags.length) emit("update:modelValue", tags);
    inputValue.value = "";
};

const onKeydown = (event: KeyboardEvent) => {
    if (event.key !== "Enter" && event.key !== ",") return;
    event.preventDefault();
    addTags();
};

const removeTag = (index: number) => {
    emit(
        "update:modelValue",
        props.modelValue.filter((_, tagIndex) => tagIndex !== index),
    );
};
</script>
