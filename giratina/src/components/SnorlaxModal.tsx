import { useEffect } from 'react';
import { Modal, Form, Input, DatePicker } from 'antd';
import dayjs from 'dayjs';

interface SnorlaxModalProps {
  open: boolean;
  onSubmit: (values: any) => void;
  onCancel: () => void;
  loading?: boolean;
}

export const SnorlaxModal = ({
  open,
  onSubmit,
  onCancel,
  loading,
}: SnorlaxModalProps) => {
  const [form] = Form.useForm();

  useEffect(() => {
    if (open) {
      form.resetFields();
    }
  }, [open, form]);

  const handleOk = async () => {
    try {
      const values = await form.validateFields();
      // Convert date to ISO string
      const payload = {
        ...values,
        release_date: values.release_date.toISOString(),
      };
      onSubmit(payload);
    } catch (error) {
      // Validation failed
    }
  };

  return (
    <Modal
      title="Create Snorlax Version"
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
        initialValues={{ release_date: dayjs() }}
      >
        <Form.Item
          name="version"
          label="Version"
          rules={[
            { required: true, message: 'Please enter version' },
            { pattern: /^\d+\.\d+\.\d+$/, message: 'Version must be in format X.Y.Z (e.g., 1.0.0)' },
          ]}
        >
          <Input placeholder="e.g., 1.0.0" />
        </Form.Item>

        <Form.Item
          name="gcs_path"
          label="GCS Path"
          rules={[
            { required: true, message: 'Please enter GCS path' },
          ]}
          extra="Folder path in GCS (e.g., Snorlax/1.0.0). The APK will be at this path + '/Snorlax.apk'"
        >
          <Input placeholder="e.g., Snorlax/1.0.0" />
        </Form.Item>

        <Form.Item
          name="release_date"
          label="Release Date"
          rules={[{ required: true, message: 'Please select release date' }]}
        >
          <DatePicker
            showTime
            style={{ width: '100%' }}
            format="YYYY-MM-DD HH:mm:ss"
          />
        </Form.Item>
      </Form>
    </Modal>
  );
};
