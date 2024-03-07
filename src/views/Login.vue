<template>
    <div>
        <div class="h-44">
            <div class="h-full w-full"></div>
            <h1>Login</h1>

            <form>
                <div class="mx-4">
                    <label for="business-tin">Business TIN</label>
                    <input placeholder="Business TIN" type="text">
                </div>
            </form>
        </div>
    </div>
</template>

<script>
export default {
    name: 'login',
}
</script>

<style>
    h1 {
        font-weight: 400;
    }
</style>