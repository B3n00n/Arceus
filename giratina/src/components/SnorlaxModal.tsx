import { useEffect, useState } from 'react';
import { Modal, Form, Input, Upload, message, Button } from 'antd';
import { InboxOutlined } from '@ant-design/icons';
import type { UploadProps } from 'antd';

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
  const [apkFile, setApkFile] = useState<File | null>(null);

  const resetUpload = () => {
    setApkFile(null);
  };

  useEffect(() => {
    if (open) {
      form.resetFields();
      resetUpload();
    }
  }, [open, form]);

  const handleOk = async () => {
    try {
      const values = await form.validateFields();

      if (!apkFile) {
        message.error('APK file is required');
        return;
      }

      values.apkFile = apkFile;
      onSubmit(values);
    } catch {
      // Validation failed
    }
  };

  const uploadProps: UploadProps = {
    beforeUpload: (file) => {
      const isApk = file.name.toLowerCase().endsWith('.apk');
      if (!isApk) {
        message.error('Only APK files are allowed');
        return Upload.LIST_IGNORE;
      }

      setApkFile(file);
      return false;
    },
    accept: '.apk',
    maxCount: 1,
    showUploadList: false,
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
      <Form form={form} layout="vertical">
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

        <Form.Item label="Snorlax APK" required>
          {!apkFile ? (
            <Upload.Dragger {...uploadProps}>
              <p className="ant-upload-drag-icon">
                <InboxOutlined />
              </p>
              <p className="ant-upload-text">Click or drag APK file to upload</p>
            </Upload.Dragger>
          ) : (
            <div style={{
              padding: '20px',
              border: '1px solid #424242',
              borderRadius: '8px',
              backgroundColor: '#1a1a1a',
              textAlign: 'center',
            }}>
              <div style={{ marginBottom: '12px', fontSize: '14px' }}>
                {apkFile.name}
              </div>
              <Button onClick={resetUpload}>
                Change File
              </Button>
            </div>
          )}
        </Form.Item>
      </Form>
    </Modal>
  );
};
