import { fireEvent, render } from '@testing-library/vue';
import { describe, expect, it } from 'vitest';
import Modal from '../../components/Modal.vue';

describe('Modal', () => {
  it('emits submit when the submit action is clicked', async () => {
    const { emitted, getByRole } = render(Modal, {
      props: {
        modelValue: true,
        title: 'Test modal',
      },
    });

    await fireEvent.click(getByRole('button', { name: 'Save' }));

    expect(emitted().submit).toHaveLength(1);
  });

  it('can hide the submit action in read-only mode', () => {
    const { getByRole, queryByRole } = render(Modal, {
      props: {
        modelValue: true,
        title: 'Read-only modal',
        showSubmit: false,
      },
    });

    expect(queryByRole('button', { name: 'Save' })).toBeNull();
    expect(getByRole('button', { name: 'Cancel' })).toBeTruthy();
  });
});
