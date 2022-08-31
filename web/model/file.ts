import { useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { createContainer } from '../common';

export const FileContainer = createContainer(() => {
    const [hovered, setHovered] = useState<boolean>(false);

    listen('file-drop', () => {
        setHovered(true);
    })

    return {
        hovered,
    }
})
