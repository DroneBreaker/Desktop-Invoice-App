<template>
    <div>
        <button class="bg-blue-600 mt-4 p-3 px-4 rounded-md text-white w-40 mb-4 font-bold">
            {{ props.buttonText }}
        </button>
    </div>
</template>

<script setup>
/**
 * 
*/
    const buttonLocation = "{{ props.buttonText.location }}";
    const props = defineProps({
        buttonText: {
            type: String,
            required: true,
            default: "Unknown Button",
            location: String
        }
    })
</script>

<style lang="scss" scoped>

</style>