import { useEffect } from 'react';
import { Modal, Form, Input } from 'antd';
import type { Game } from '../types';

interface GameModalProps {
  open: boolean;
  mode: 'create' | 'edit';
  game?: Game;
  onSubmit: (values: any) => void;
  onCancel: () => void;
  loading?: boolean;
}

export const GameModal = ({
  open,
  mode,
  game,
  onSubmit,
  onCancel,
  loading,
}: GameModalProps) => {
  const [form] = Form.useForm();

  useEffect(() => {
    if (open) {
      if (mode === 'edit' && game) {
        form.setFieldsValue({
          name: game.name,
        });
      } else {
        form.resetFields();
      }
    }
  }, [open, mode, game, form]);

  const handleOk = async () => {
    try {
      const values = await form.validateFields();
      onSubmit(values);
    } catch (error) {
      // Validation failed
    }
  };

  return (
    <Modal
      title={mode === 'create' ? 'Create Game' : 'Edit Game'}
      open={open}
      onOk={handleOk}
      onCancel={onCancel}
      confirmLoading={loading}
      destroyOnHidden
      width={500}
    >
      <Form form={form} layout="vertical">
        <Form.Item
          name="name"
          label="Game Name"
          rules={[
            { required: true, message: 'Please enter game name' },
            { min: 2, message: 'Name must be at least 2 characters' },
            { max: 255, message: 'Name must not exceed 255 characters' },
          ]}
        >
          <Input placeholder="Enter game name" />
        </Form.Item>
      </Form>
    </Modal>
  );
};
