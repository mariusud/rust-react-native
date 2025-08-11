import { registerWebModule, NativeModule } from 'expo';

import { ChangeEventPayload } from './RustModule.types';

type RustModuleEvents = {
  onChange: (params: ChangeEventPayload) => void;
}

class RustModule extends NativeModule<RustModuleEvents> {
  PI = Math.PI;
  async setValueAsync(value: string): Promise<void> {
    this.emit('onChange', { value });
  }
  hello() {
    return 'Hello world! 👋';
  }
};

export default registerWebModule(RustModule, 'RustModule');
