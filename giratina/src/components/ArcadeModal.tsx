import { useEffect } from 'react';
import { Modal, Form, Input, Select } from 'antd';
import type { Arcade } from '../types';
import { useChannels } from '../hooks/useChannels';

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
  const { data: channels = [], isLoading: channelsLoading } = useChannels();

  const channelOptions = channels.map((channel) => ({
    label: channel.name.charAt(0).toUpperCase() + channel.name.slice(1),
    value: channel.id,
  }));

  useEffect(() => {
    if (open) {
      if (mode === 'edit' && arcade) {
        form.setFieldsValue({
          name: arcade.name,
          machine_id: arcade.machine_id,
          status: arcade.status,
          channel_id: arcade.channel_id,
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

  const validateMachineId = (_: any, value: string) => {
    if (!value) {
      return Promise.reject(new Error('Machine ID is required'));
    }
    // Machine ID is a 32-character hexadecimal string
    const machineIdRegex = /^[0-9a-fA-F]{32}$/;
    if (!machineIdRegex.test(value)) {
      return Promise.reject(new Error('Invalid machine ID format (must be 32 hexadecimal characters)'));
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
      destroyOnClose
      width={600}
    >
      <Form
        form={form}
        layout="vertical"
        initialValues={{ status: 'active', channel_id: 1 }}
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
          name="machine_id"
          label="Machine ID"
          rules={[{ validator: validateMachineId }]}
        >
          <Input
            placeholder="32 character hex (no '-')"
            disabled={mode === 'edit'}
            style={{ fontFamily: 'monospace', textTransform: 'lowercase' }}
            maxLength={32}
          />
        </Form.Item>

        <Form.Item
          name="status"
          label="Status"
          rules={[{ required: true, message: 'Please select status' }]}
        >
          <Select options={ARCADE_STATUSES} />
        </Form.Item>

        <Form.Item
          name="channel_id"
          label="Release Channel"
          rules={[{ required: true, message: 'Please select release channel' }]}
        >
          <Select
            options={channelOptions}
            loading={channelsLoading}
            placeholder="Select release channel"
          />
        </Form.Item>
      </Form>
    </Modal>
  );
};
