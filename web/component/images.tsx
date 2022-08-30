import React from 'react';
import { DEFAULT_DARG_IMAGE_TIPS } from '@common';
import './images.less';

export default () => {
    return (
        <div className='images-container'>
            <div className='images-inner'>
                <div className='images-content'>
                    {DEFAULT_DARG_IMAGE_TIPS}
                </div>
            </div>
        </div>
    )
}