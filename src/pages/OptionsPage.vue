<script setup>
    import { ref } from 'vue';
    import { useI18n } from 'vue-i18n';
    import { useStore } from '@/stores/options';
    import { useRouter } from 'vue-router';
    import { useConfirm, useAlert } from 'balm-ui';
    import { open } from '@tauri-apps/api/dialog';
    import { documentDir } from '@tauri-apps/api/path';
    import { invoke } from '@tauri-apps/api'
    
    const { t } = useI18n();
    const $confirm = useConfirm();
    const $alert = useAlert();
    const optionsStore = useStore();
    const enginePath = ref(optionsStore.selectedEnginePath ?? '');

    const router = useRouter();

    function setOptionsFromConfig(configObject) {
        optionsStore.selectedEnginePath = configObject.engine_path;
    }

    function getConfigObjectFromOptions() {
        return {
            'engine_path': optionsStore.selectedEnginePath ?? '',
        };
    }

    async function saveSettings() {
        try {
            const config = getConfigObjectFromOptions();
            await invoke('save_settings', {settingsJson: JSON.stringify(config)})
        }
        catch (ex) {
            console.error(ex);
        }
    }

    function commitChanges() {
        optionsStore.setSelectedEnginePath(enginePath.value);
        saveSettings();
    }

    function doValidate() {
        commitChanges();
        router.back();
    }

    function doCancel() {
        router.back();
    }

    function handleValidate() {
        $confirm({message: t('pages.options.dialogs.validate-confirmation'),
        acceptText: t('dialogs.ok'), cancelText: t('dialogs.cancel')}).then((result) => {
            if (result) doValidate();
        });
    }

    function handleCancel() {
        $confirm({message: t('pages.options.dialogs.cancel-confirmation'),
        acceptText: t('dialogs.ok'), cancelText: t('dialogs.cancel')}).then((result) => {
            if (result) doCancel();
        });
    }

    async function validateEnginePath(enginePath) {
        const result = await invoke('check_uci_engine_path', {absolutePath: enginePath});
        return result === undefined;
    }

    async function letUserSelectEnginePath() {
        const documentDirPath = await documentDir();
        const selected = await open({
            multiple: false,
            defaultPath: documentDirPath,
            directory: false,
            recursive: true,
            title: t('pages.options.dialogs.select-engine-title')
        });
        if (!selected) return;
        
        try {
            await validateEnginePath(selected);
            enginePath.value = selected;
        }
        catch (ex) {
            console.error(ex);
            $alert({
                message: t('pages.options.dialogs.not-uci-engine'),
                state: 'error',
            })
        }
    }

    defineExpose({});
</script>

<template>
    <div class="root">
        <div class="field-line">
            <label for="enginePathField">
                {{ t('pages.options.fields.engine-path-label')}}
            </label>
            <input type="text" id="enginePathField" v-model="enginePath" />
            <ui-button raised id="enginePathButton" @click="letUserSelectEnginePath">{{ t('pages.options.fields.engine-path-button') }}</ui-button>
        </div>
        <div class="validation-line">
            <ui-button id="validate-btn" raised @click="handleValidate">{{t('dialogs.ok')}}</ui-button>
            <ui-button id="cancel-btn" raised @click="handleCancel">{{t('dialogs.cancel')}}</ui-button>
        </div>   
    </div>    
</template>

<style scoped>
.root {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    height: 97vh;
    color: blue;
}

.field-line, .validation-line {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    padding: 10px 20px;
}

.validation-line {
    justify-content: space-evenly;
}

#enginePathField {
    margin: 0 20px;
    width: 380px;
}

#enginePathButton {
    background-color: blue;
}

#validate-btn {
    background-color: green;
}

#cancel-btn {
    background-color: red;
}
</style>