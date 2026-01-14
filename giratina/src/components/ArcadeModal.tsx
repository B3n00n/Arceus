import { useEffect } from 'react';
import { Modal, Form, Input, Select } from 'antd';
import type { Arcade } from '../types';

interface ArcadeModalProps {
  open: boolean;
  mode: 'create' | 'edit';
  arcade?: Arcade;
  onSubmit: (values: any) => void;
  onCancel: () => void;
  loading?: boolean;
}

const ARCADE_STATUSES = [
  { label: 'Active', value: 'active' },
  { label: 'Inactive', value: 'inactive' },
  { label: 'Maintenance', value: 'maintenance' },
];

export const ArcadeModal = ({
  open,
  mode,
  arcade,
  onSubmit,
  onCancel,
  loading,
}: ArcadeModalProps) => {
  const [form] = Form.useForm();

  useEffect(() => {
    if (open) {
      if (mode === 'edit' && arcade) {
        form.setFieldsValue({
          name: arcade.name,
          mac_address: arcade.mac_address,
          status: arcade.status,
        });
      } else {
        form.resetFields();
      }
    }
  }, [open, mode, arcade, form]);

  const handleOk = async () => {
    try {
      const values = await form.validateFields();
      onSubmit(values);
    } catch {
      // Validation failed
    }
  };

  const validateMacAddress = (_: any, value: string) => {
    if (!value) {
      return Promise.reject(new Error('MAC address is required'));
    }
    const macRegex = /^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$/;
    if (!macRegex.test(value)) {
      return Promise.reject(new Error('Invalid MAC address format (e.g., AA:BB:CC:DD:EE:FF)'));
    }
    return Promise.resolve();
  };

  return (
    <Modal
      title={mode === 'create' ? 'Create Arcade' : 'Edit Arcade'}
      open={open}
      onOk={handleOk}
      onCancel={onCancel}
      confirmLoading={loading}
      destroyOnHidden
      width={600}
    >
      <Form
        form={form}
        layout="vertical"
        initialValues={{ status: 'active' }}
      >
        <Form.Item
          name="name"
          label="Arcade Name"
          rules={[
            { required: true, message: 'Please enter arcade name' },
            { min: 3, message: 'Name must be at least 3 characters' },
            { max: 255, message: 'Name must not exceed 255 characters' },
          ]}
        >
          <Input placeholder="Enter arcade name" />
        </Form.Item>

        <Form.Item
          name="mac_address"
          label="MAC Address"
          rules={[{ validator: validateMacAddress }]}
        >
          <Input
            placeholder="AA:BB:CC:DD:EE:FF"
            disabled={mode === 'edit'}
            style={{ textTransform: 'uppercase' }}
          />
        </Form.Item>

        <Form.Item
          name="status"
          label="Status"
          rules={[{ required: true, message: 'Please select status' }]}
        >
          <Select options={ARCADE_STATUSES} />
        </Form.Item>
      </Form>
    </Modal>
  );
};
